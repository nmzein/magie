#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_lines)]

mod api;
mod db;
mod io;
mod log;

#[cfg(test)]
mod tests;

use axum::{
    extract::DefaultBodyLimit,
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    middleware::{self},
    routing::{delete, get, patch, post},
    Router,
};
use log::logging_middleware;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Override the temporary directory to get around issue
    // of crossing mount points on some Linux distros.
    env::set_var("TMPDIR", "./tmp");

    // Load environment variables from .env file.
    dotenvy::dotenv().expect("Could not load .env file.");

    let frontend_url = &fetch_env_var("PUBLIC_FRONTEND_URL");
    let backend_url = &fetch_env_var("PUBLIC_BACKEND_URL");
    let http_scheme = &fetch_env_var("PUBLIC_HTTP_SCHEME");

    let listener = TcpListener::bind(backend_url)
        .await
        .expect("Could not bind a TcpListener to the backend port.");

    let frontend_url = format!("{http_scheme}://{frontend_url}");
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(
            frontend_url
                .parse::<HeaderValue>()
                .expect("Could not parse frontend url."),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_headers([CONTENT_TYPE]);

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

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::disable());

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}

fn fetch_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is not set."))
}
