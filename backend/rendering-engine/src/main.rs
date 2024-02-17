#![deny(clippy::all)]
#![warn(
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod api;
mod db;
mod decoders;
mod generators;
mod io;
mod structs;
mod traits;

use crate::structs::AppState;

use std::{collections::HashMap, env};

use axum::{
    Extension,
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let domain = env::var("DOMAIN").expect("DOMAIN is not set.");
    let frontend_port = env::var("FRONTEND_PORT").expect("FRONTEND_PORT is not set.");
    let backend_port = env::var("BACKEND_PORT").expect("BACKEND_PORT is not set.");

    let pool = db::connect(&database_url).await.unwrap();

    let state = AppState {
        pool,
        generators: HashMap::new(),
    };

    let backend_url = format!("{}:{}", domain, backend_port);
    let listener = tokio::net::TcpListener::bind(backend_url)
        .await
        .unwrap();

    // TODO: Allow for http/https.
    let frontend_url = format!("http://{}:{}", domain, frontend_port);
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    // TODO: Rename endpoints and add to global .env.
    let app = Router::new()
        .route("/api/annotations", post(api::annotations::annotations))
        .route("/api/connect", get(api::chunks::connect))
        .route("/api/delete", post(api::delete::delete))
        .route("/api/annotation-generators", get(api::generators::generators))
        .route("/api/metadata", post(api::metadata::metadata))
        .route("/api/image-list", get(api::stores::stores))
        .route("/api/upload", post(api::upload::upload))
        .layer(cors)
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(state));


    axum::serve(listener, app).await.unwrap();
}
