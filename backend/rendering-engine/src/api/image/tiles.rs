use crate::api::common::*;
use crate::types::TileRequest;
use axum::extract::{
    ws::{Message, WebSocket},
    WebSocketUpgrade,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub async fn websocket(
    ws: WebSocketUpgrade,
    Extension(state): Extension<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        tiles(socket, Extension(state)).await;
    })
}

// TODO: Send error messages to frontend.
// TODO: Capture large rectangles of selections rather than individual tiles.
async fn tiles(socket: WebSocket, Extension(conn): Extension<AppState>) {
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
        let sender = sender.clone();
        let conn = Arc::clone(&conn);

        tokio::spawn(async move {
            let tile_request = match serde_json::from_str::<TileRequest>(&message) {
                Ok(tile_request) => tile_request,
                Err(e) => {
                    log(
                        StatusCode::BAD_REQUEST,
                        &format!("Failed to parse tile request: {message}."),
                        Some(e),
                    );

                    return;
                }
            };

            // TODO: Cache in an in-memory HashMap.
            let (_, path) = match crate::db::image::get(tile_request.id, Arc::clone(&conn)) {
                Ok(paths) => paths,
                Err(e) => {
                    log(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!(
                            "Failed to retrieve paths for image with id: {}.",
                            tile_request.id
                        ),
                        Some(e),
                    );

                    return;
                }
            };

            // TODO: Remove hardcoding, import from consts.
            let encoded_img_path = path.join("encoded_image.zarr");
            let tile = match crate::io::retrieve(&encoded_img_path, &tile_request).await {
                Ok(tile) => tile,
                Err(e) => {
                    log(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!(
                            "Failed to retrieve tile for image with id: {}.",
                            tile_request.id
                        ),
                        Some(e),
                    );

                    return;
                }
            };

            let _ = sender.send(Message::Binary(tile)).await.map_err(|e| {
                log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!(
                        "Failed to send tile for image with id: {}.",
                        tile_request.id
                    ),
                    Some(e),
                );

                return;
            });
        });
    }
}
