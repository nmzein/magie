#![deny(clippy::all)]
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod api;
mod db;
mod io;
mod types;

use axum::{
    extract::DefaultBodyLimit,
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{delete, get, post},
    Extension, Router,
};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenvy::dotenv().expect("Could not load .env file.");

    let database_path = &fetch_env_var("DATABASE_PATH");
    let database_url = &fetch_env_var("DATABASE_URL");
    let domain = &fetch_env_var("PUBLIC_DOMAIN");
    let frontend_port = &fetch_env_var("PUBLIC_FRONTEND_PORT");
    let backend_port = &fetch_env_var("PUBLIC_BACKEND_PORT");
    let http_scheme = &fetch_env_var("PUBLIC_HTTP_SCHEME");

    // Directory routes.
    let directory_create_url = &fetch_env_var("PUBLIC_DIRECTORY_CREATE_SUBDIR");
    let directory_delete_url = &fetch_env_var("PUBLIC_DIRECTORY_DELETE_SUBDIR");
    let directory_rename_url = &fetch_env_var("PUBLIC_DIRECTORY_RENAME_SUBDIR");
    let directory_move_url = &fetch_env_var("PUBLIC_DIRECTORY_MOVE_SUBDIR");

    // Image routes.
    let image_upload_url = &fetch_env_var("PUBLIC_IMAGE_UPLOAD_SUBDIR");
    let image_delete_url = &fetch_env_var("PUBLIC_IMAGE_DELETE_SUBDIR");
    let image_metadata_url = &fetch_env_var("PUBLIC_IMAGE_METADATA_SUBDIR");
    let image_annotation_url = &fetch_env_var("PUBLIC_IMAGE_ANNOTATIONS_SUBDIR");
    let image_tiles_url = &fetch_env_var("PUBLIC_IMAGE_TILES_SUBDIR");

    // General routes.
    let registry_url = &fetch_env_var("PUBLIC_REGISTRY_SUBDIR");
    let generators_url = &fetch_env_var("PUBLIC_GENERATORS_SUBDIR");

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
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        // Directory routes.
        .route(directory_create_url, post(api::directory::create::create))
        // TODO: Reflect this in env file.
        .route(
            &format!("{}/{}", directory_delete_url, ":id"),
            delete(api::directory::delete::delete),
        )
        .route(directory_rename_url, post(api::directory::rename::rename))
        .route(directory_move_url, post(api::directory::r#move::r#move))
        // Image routes.
        .route(image_upload_url, post(api::image::upload::upload))
        .route(image_delete_url, post(api::image::delete::delete))
        .route(image_metadata_url, post(api::image::metadata::metadata))
        .route(
            image_annotation_url,
            post(api::image::annotations::annotations),
        )
        .route(image_tiles_url, get(api::image::tiles::websocket))
        // General routes.
        .route(registry_url, get(api::registry::registry))
        .route(generators_url, get(api::generators::generators))
        .layer(cors)
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(Arc::new(Mutex::new(conn))));

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}

fn fetch_env_var(name: &str) -> String {
    env::var(name).expect(&format!("{name} is not set."))
}
