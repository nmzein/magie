mod image;
mod io;
mod structs;

use crate::structs::{ImageState, ImageMetadata, ImageSelection, State};

use std::path::PathBuf;
use std::collections::BTreeMap;
use std::fs;
use std::fmt::Display;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, WebSocketUpgrade
    },
    response::{Json, IntoResponse, Response},
    http::{Method, StatusCode},
    routing::{get, post},
    Router, Server
};

use futures_util::{SinkExt, StreamExt};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let mut state: State = BTreeMap::new();

    // ** Example Only ** //
    let example_state = ImageState {
        image_path: PathBuf::from("store/image/image.tiff"),
        datastore_path: PathBuf::from("store/image/image.h5"),
        image_metadata: ImageMetadata {
            cols: 44,
            rows: 35
        }
    };

    state.insert(String::from("image"), example_state);
    // ****************** //

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/connect", get(connect))
        .route("/api/process", post(process))
        .route("/api/metadata", post(metadata))
        .route("/api/delete", post(delete))
        .layer(cors)
        .layer(Extension(state));

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn connect(socket_upgrader: WebSocketUpgrade, Extension(state): Extension<State>) -> impl IntoResponse {
    socket_upgrader.on_upgrade(|socket| async {
        render(socket, Extension(state)).await;
    })
}

async fn render(socket: WebSocket, Extension(state): Extension<State>) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(Message::Text(message))) = receiver.next().await {
        if let Ok(selection) = serde_json::from_str::<ImageSelection>(&message) {
            // ** Example Only ** //
            let name = "image";
            // ****************** //
            
            println!("Received selection: {:?}", selection);

            match io::read_hdf5(
                &state[name].datastore_path,
                &selection,
                &state[name].image_metadata
            ) {
                Ok(tiles) => {
                    println!("Sending {} tiles.", tiles.len());
                    println!("ImageState Metadata: {:?}", &state[name].image_metadata);
                    for tile in tiles {
                        let _ = sender.send(Message::Binary(tile)).await.map_err(|err| {
                            eprintln!("Error sending tile: {}", err);
                        });
                    }
                }
                Err(err) => {
                    eprintln!("Error reading from datastore: {}", err);
                }
            }
        } else {
            eprintln!("Error deserialising selection.");
        }
    }
}

async fn metadata(Extension(state): Extension<State>) -> Response {
    // ** Example Only ** //
    let name = "image";
    // ****************** //
    
    Json(&state[name].image_metadata).into_response()
}

async fn process(Extension(mut state): Extension<State>) -> Response {
    // ** Example Only ** //
    let image = "image.tiff";
    // ****************** //
    
    // Strip file extension.
    let name = String::from(image.split('.').collect::<Vec<&str>>()[0]);

    if state.contains_key(&name) {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            format!("ImageState with name {} already exists. Consider deleting it from the list first.", name).as_str(),
            None
        );
    }
    
    let image_path = PathBuf::from(format!("store/{}/{}", name, image));
    let datastore_path = PathBuf::from(format!("store/{}/{}.h5", name, name));

    match image::process(
        &image_path,
        &datastore_path,
    ) {
        Ok(image_metadata) => {
            let image = ImageState { image_path, datastore_path, image_metadata };
            state.insert(name, image);
            // TODO: Store in SQLite database.
            log_respond::<String>(StatusCode::OK, "Successfully processed image.", None)
        },
        Err(err) => log_respond(StatusCode::INTERNAL_SERVER_ERROR, "Failed to process the image.", Some(err))
    }
}

async fn delete(Extension(mut state): Extension<State>) -> Response {
    // ** Example Only ** //
    let name = "image";
    // ****************** //

    let dir_path = PathBuf::from("store/".to_owned() + name);
    
    // Remove directory.
    let _  = fs::remove_dir_all(dir_path).map_err(|err| {
        return log_respond(StatusCode::INTERNAL_SERVER_ERROR, "Could not delete directory.", Some(err));
    });
    
    // TODO: Remove from SQLite database.

    // Remove from map.
    state.remove(name);

    log_respond::<String>(StatusCode::OK, "Successfully deleted image entry.", None)
}

fn log_respond<T: Display>(status_code: StatusCode, message: &str, details: Option<T>) -> Response {
    if status_code.is_success() {
        println!("Ok: {}", message);
        if let Some(details) = details {
            println!("Details: {}", details);
        }
    } else {
        eprintln!("Error <{}>: {}", status_code, message);
        if let Some(details) = details {
            eprintln!("Details: {}", details);
        }
    }

    (status_code, String::from(message)).into_response()
}