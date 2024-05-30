#![deny(clippy::all)]
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod api;
mod consts;
mod db;
mod io;
mod types;

use axum::{
    extract::DefaultBodyLimit,
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::{
    env,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv().ok();

    let database_url = &fetch_env_var("DATABASE_URL");
    let domain = &fetch_env_var("PUBLIC_DOMAIN");
    let frontend_port = &fetch_env_var("PUBLIC_FRONTEND_PORT");
    let backend_port = &fetch_env_var("PUBLIC_BACKEND_PORT");
    let http_scheme = &fetch_env_var("PUBLIC_HTTP_SCHEME");
    let annotation_url = &fetch_env_var("PUBLIC_ANNOTATIONS_SUBDIR");
    let delete_url = &fetch_env_var("PUBLIC_DELETE_SUBDIR");
    let generators_url = &fetch_env_var("PUBLIC_GENERATORS_SUBDIR");
    let metadata_url = &fetch_env_var("PUBLIC_METADATA_SUBDIR");
    let stores_url = &fetch_env_var("PUBLIC_STORES_SUBDIR");
    let websocket_url = &fetch_env_var("PUBLIC_WEBSOCKET_SUBDIR");
    let upload_url = &fetch_env_var("PUBLIC_UPLOAD_SUBDIR");

    let conn = db::connect(database_url)
        .await
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
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route(annotation_url, post(api::annotations::annotations))
        .route(delete_url, post(api::delete::delete))
        .route(generators_url, get(api::generators::generators))
        .route(metadata_url, post(api::metadata::metadata))
        .route(stores_url, get(api::stores::stores))
        .route(websocket_url, get(api::tiles::websocket))
        .route(upload_url, post(api::upload::upload))
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
