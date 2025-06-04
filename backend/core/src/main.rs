#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_lines)]

mod api;
mod constants;
mod db;
mod io;
mod log;
mod middleware;
mod types;

use crate::{
    constants::{LOCAL_DATABASES_PATH, LOCAL_STORES_PATH, REGISTRY_PATH},
    types::{database::DatabaseManager, socket::ClientSocketManager},
};
use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
};
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower::builder::ServiceBuilder;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let port: &str = &env::var("PUBLIC_PORT").expect("PUBLIC_PORT environment variable not set");
    let container: bool = env::var("CONTAINER").unwrap_or("false".into()) == "true";

    let backend_url: &str = if container {
        &format!("0.0.0.0:{port}")
    } else {
        &format!("localhost:{port}")
    };

    let tmp_dir = PathBuf::from(LOCAL_STORES_PATH).join("tmp");

    // TODO: Move to Nix flake.
    // SAFETY: Environment access only happens in single-threaded code.
    // Override the temporary directory to get around issue
    // of crossing mount points on some Linux distros.
    unsafe { env::set_var("TMPDIR", &tmp_dir) };

    // TODO: Move to Nix flake.
    // Create the necessary directories.
    if !Path::new(LOCAL_STORES_PATH).exists() {
        println!("Creating local stores directory at: {LOCAL_STORES_PATH}");
        fs::create_dir_all(LOCAL_STORES_PATH).expect("Could not create local stores directory");

        println!("Creating local temporary file directory at: {tmp_dir:#?}");
        fs::create_dir_all(&tmp_dir).expect("Could not create local temporary file directory");
    }

    if !Path::new(LOCAL_DATABASES_PATH).exists() {
        println!("Creating local databases directory at: {LOCAL_DATABASES_PATH}");
        fs::create_dir_all(LOCAL_DATABASES_PATH)
            .expect("Could not create local databases directory");
    }

    if !Path::new(REGISTRY_PATH).exists() {
        println!("Creating registry database at: {REGISTRY_PATH}");
        fs::File::create(REGISTRY_PATH).expect("Could not create registry database file");
    }

    let listener = TcpListener::bind(backend_url)
        .await
        .expect("Could not bind a TcpListener to the backend port.");

    let directory_routes = Router::new()
        .route("/{parent_id}/{name}", post(api::directory::create::create))
        .route("/{directory_id}", delete(api::directory::delete::delete))
        // TODO: Make this endpoint accept rename too
        .route("/{directory_id}", patch(api::directory::r#move::r#move));

    let image_routes = Router::new()
        .route("/{parent_id}/{name}", post(api::image::upload::upload))
        .route("/{image_id}", delete(api::image::delete::delete))
        // TODO: Make this endpoint accept rename too
        .route("/{image_id}", patch(api::image::r#move::r#move))
        .route(
            "/{image_id}/properties",
            get(api::image::properties::properties),
        )
        .route(
            "/{image_id}/thumbnail",
            get(api::image::thumbnail::thumbnail),
        )
        .route(
            "/{image_id}/annotations/{annotation_layer_id}",
            get(api::image::annotations::annotations),
        );

    let store_routes = Router::new().route("/{store_id}", get(api::store::get::get));

    let api_routes = Router::new()
        .nest("/directory/{store_id}", directory_routes)
        .nest("/image/{store_id}", image_routes)
        .nest("/store", store_routes)
        .route("/registry", get(api::registry::registry))
        .route("/generators", get(api::generators::generators))
        .route("/websocket", get(api::websocket::websocket));

    let static_routes = ServiceBuilder::new().service(ServeDir::new("_static"));

    let mut app = Router::new()
        .fallback_service(static_routes)
        .nest("/api", api_routes)
        .layer(axum::middleware::from_fn(crate::middleware::logging))
        .layer(axum::middleware::from_fn(crate::middleware::authentication))
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(Arc::new(
            DatabaseManager::connect().expect("Could not connect to the databases."),
        )))
        .layer(Extension(Arc::new(ClientSocketManager::default())));

    // Allow CORS from dev frontend server.
    #[cfg(debug_assertions)]
    {
        use axum::http::{HeaderValue, Method, header::CONTENT_TYPE};
        use tower_http::cors::CorsLayer;

        let frontend_port: &str =
            &env::var("DEV_FRONTEND_PORT").expect("DEV_FRONTEND_PORT environment variable not set");

        let frontend_url: &str = &format!("http://localhost:{frontend_port}");

        let cors = CorsLayer::new()
            .allow_origin(
                frontend_url
                    .parse::<HeaderValue>()
                    .expect("Could not parse frontend url."),
            )
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
            .allow_headers([CONTENT_TYPE]);

        app = app.layer(cors);
    }

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}
