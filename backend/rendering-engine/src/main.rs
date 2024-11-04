#![deny(clippy::all)]
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod api;
mod db;
mod io;
mod log;
mod types;

#[cfg(test)]
mod tests;

use axum::{
    extract::DefaultBodyLimit,
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    middleware::{self},
    routing::{delete, get, patch, post},
    Extension, Router,
};
use log::logging_middleware;
use std::{
    env,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Override the temporary directory to get around issue
    // of crossing mount points on some Linux distros.
    env::set_var("TMPDIR", "./tmp");

    // Load environment variables from .env file.
    dotenvy::dotenv().expect("Could not load .env file.");

    // TODO: Store in a STORES db.
    let database_path = "./state/db.sqlite";
    let database_url = "sqlite://../state/db.sqlite";

    let domain = &fetch_env_var("PUBLIC_DOMAIN");
    let frontend_port = &fetch_env_var("PUBLIC_FRONTEND_PORT");
    let backend_port = &fetch_env_var("PUBLIC_BACKEND_PORT");
    let http_scheme = &fetch_env_var("PUBLIC_HTTP_SCHEME");

    let conn = db::general::connect(database_path, database_url)
        .expect("Could not establish a connection to the state database.");

    let backend_url = format!("{domain}:{backend_port}");
    let listener = TcpListener::bind(backend_url)
        .await
        .expect("Could not bind a TcpListener to the backend port.");

    let frontend_url = format!("{http_scheme}://{domain}:{frontend_port}");
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(
            frontend_url
                .parse::<HeaderValue>()
                .expect("Could not parse frontend url."),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_headers([CONTENT_TYPE]);

    let directory_routes = Router::new()
        .route("/:parent_id/:name", post(api::directory::create::create))
        .route("/:id", delete(api::directory::delete::delete))
        // TODO: Make this endpoint accept rename too
        .route("/:id", patch(api::directory::r#move::r#move));

    let image_routes = Router::new()
        .route("/:parent_id/:name", post(api::image::upload::upload))
        .route("/:id", delete(api::image::delete::delete))
        // TODO: Make this endpoint accept rename too
        .route("/:id", patch(api::image::r#move::r#move))
        .route("/:id/properties", get(api::image::properties::properties))
        .route("/:id/thumbnail", get(api::image::thumbnail::thumbnail))
        .route(
            "/:image_id/annotations/:annotation_layer_id",
            get(api::image::annotations::annotations),
        );

    let api_routes = Router::new()
        .nest("/directory", directory_routes)
        .nest("/image", image_routes)
        .route("/registry", get(api::registry::registry))
        .route("/generators", get(api::generators::generators))
        .route("/websocket", get(api::image::tiles::websocket));

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(Arc::new(Mutex::new(conn))));

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}

fn fetch_env_var(name: &str) -> String {
    env::var(name).expect(&format!("{name} is not set."))
}
