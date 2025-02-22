#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::too_many_lines)]

mod api;
mod db;
mod io;
mod log;

#[cfg(test)]
mod tests;

use axum::{
    Extension, Router,
    extract::{DefaultBodyLimit, ws::Message},
    http::{HeaderValue, Method, header::CONTENT_TYPE},
    middleware::{self},
    routing::{delete, get, patch, post},
};
use dashmap::DashMap;
use log::logging_middleware;
use std::{env, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};
use tower_http::cors::CorsLayer;

type AppState = Arc<State>;
type Broadcast = broadcast::Sender<Message>;
type Connections = DashMap<u32, mpsc::Sender<Message>>;

#[derive(Debug, Clone)]
struct State {
    broadcast: Broadcast,
    connections: Connections,
}

impl Default for State {
    fn default() -> Self {
        Self {
            broadcast: broadcast::channel(1024).0,
            connections: DashMap::new(),
        }
    }
}

impl State {
    fn add_connection(&self, user_id: u32, sender: mpsc::Sender<Message>) {
        self.connections.insert(user_id, sender);
    }

    fn remove_connection(&self, user_id: u32) {
        self.connections.remove(&user_id);
    }

    // Send to specific user.
    async fn send(&self, user_id: u32, message: Message) {
        if let Some(sender) = self.connections.get(&user_id) {
            let _ = sender.send(message).await;
        }
    }

    // Broadcast to all users.
    async fn broadcast(&self, msg: Message) {
        let _ = self.broadcast.send(msg);
    }
}

#[tokio::main]
async fn main() {
    // SAFETY: Environment access only happens in single-threaded code.
    // Override the temporary directory to get around issue
    // of crossing mount points on some Linux distros.
    unsafe { env::set_var("TMPDIR", "../stores/tmp") };

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
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(AppState::default()));

    axum::serve(listener, app)
        .await
        .expect("Could not serve the backend.");
}

fn fetch_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is not set."))
}
