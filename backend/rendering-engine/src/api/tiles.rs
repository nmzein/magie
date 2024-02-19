use crate::api::common::*;
use crate::structs::{ImageState, TileRequest};
use axum::extract::{
    ws::{Message, WebSocket},
    WebSocketUpgrade,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub async fn websocket(
    ws: WebSocketUpgrade,
    Extension(AppState { current_image, .. }): Extension<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        tiles(socket, current_image).await;
    })
}

// TODO: Send error messages to frontend.
async fn tiles(socket: WebSocket, current_image: Arc<Mutex<Option<ImageState>>>) {
    let (mut sink, mut stream) = socket.split();
    // Credit: https://gist.github.com/hexcowboy/8ebcf13a5d3b681aa6c684ad51dd6e0c
    // Create an mpsc channel so we can send messages to the sink from multiple threads.
    let (sender, mut receiver) = mpsc::channel::<Message>(4);

    // Spawn a task that forwards messages from the mpsc receiver to the websocket sink.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if sink.send(message.into()).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(Message::Text(message))) = stream.next().await {
        let current_image = Arc::clone(&current_image);
        let Some(current_image) = current_image.lock().unwrap().clone() else {
            #[cfg(feature = "log")]
            log::<()>(
                StatusCode::BAD_REQUEST,
                "Image metadata must first be fetched before requesting tiles.",
                None,
            );

            continue;
        };
        let sender = sender.clone();

        tokio::spawn(async move {
            let Ok(tile_request) = serde_json::from_str::<TileRequest>(&message) else {
                #[cfg(feature = "log")]
                log::<()>(
                    StatusCode::BAD_REQUEST,
                    &format!("Failed to parse tile request: {}.", message),
                    None,
                );

                return;
            };

            // #[cfg(feature = "log")]
            // log::<()>(
            //     StatusCode::ACCEPTED,
            //     &format!("Received request for tile: {:?}.", tile_request),
            //     None,
            // );

            let Ok(tile) =
                crate::io::retrieve(&current_image.store_path.into(), tile_request.clone()).await
            else {
                #[cfg(feature = "log")]
                log::<()>(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!(
                        "Failed to retrieve tile for image with name: {}.",
                        &tile_request.image_name
                    ),
                    None,
                );

                return;
            };

            let _ = sender.send(Message::Binary(tile)).await.map_err(|e| {
                #[cfg(feature = "log")]
                log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!(
                        "Failed to send tile for image with name: {}.",
                        &tile_request.image_name
                    ),
                    Some(e),
                );
            });
        });
    }
}
