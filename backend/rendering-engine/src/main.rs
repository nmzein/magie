#![deny(clippy::all)]
#![warn(clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod api;
mod db;
mod decoders;
mod generators;
mod io;
mod structs;
mod traits;

use crate::structs::AppState;
use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let domain = &env::var("PUBLIC_DOMAIN").expect("PUBLIC_DOMAIN is not set.");
    let frontend_port =
        &env::var("PUBLIC_FRONTEND_PORT").expect("PUBLIC_FRONTEND_PORT is not set.");
    let backend_port = &env::var("PUBLIC_BACKEND_PORT").expect("PUBLIC_BACKEND_PORT is not set.");
    let http_scheme = &env::var("PUBLIC_HTTP_SCHEME").expect("PUBLIC_HTTP_SCHEME is not set.");
    let annotation_url =
        &env::var("PUBLIC_ANNOTATIONS_SUBDIR").expect("PUBLIC_ANNOTATIONS_SUBDIR is not set.");
    let delete_url = &env::var("PUBLIC_DELETE_SUBDIR").expect("PUBLIC_DELETE_SUBDIR is not set.");
    let generators_url =
        &env::var("PUBLIC_GENERATORS_SUBDIR").expect("PUBLIC_GENERATORS_SUBDIR is not set.");
    let metadata_url =
        &env::var("PUBLIC_METADATA_SUBDIR").expect("PUBLIC_METADATA_SUBDIR is not set.");
    let stores_url = &env::var("PUBLIC_STORES_SUBDIR").expect("PUBLIC_STORES_SUBDIR is not set.");
    let websocket_url =
        &env::var("PUBLIC_WEBSOCKET_SUBDIR").expect("PUBLIC_WEBSOCKET_SUBDIR is not set.");
    let upload_url = &env::var("PUBLIC_UPLOAD_SUBDIR").expect("PUBLIC_UPLOAD_SUBDIR is not set.");

    let pool = db::connect(database_url)
        .await
        .expect("Could not establish a connection to the state database.");

    let state = AppState {
        pool,
        current_image: Arc::new(Mutex::new(None)),
        generators: HashMap::new(),
    };

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
        .allow_methods([Method::GET, Method::POST]);

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
        .layer(Extension(state));

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}
