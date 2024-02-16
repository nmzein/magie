use crate::api::common::*;
use crate::structs::Selection;
use axum::extract::{
    ws::{Message, WebSocket},
    WebSocketUpgrade,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub async fn connect(ws: WebSocketUpgrade, Extension(state): Extension<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        chunks(socket, state).await;
    })
}

// TODO: Send error messages to frontend.
async fn chunks(socket: WebSocket, state: AppState) {
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
        if let Ok(selection) = serde_json::from_str::<Selection>(&message) {
            log::<String>(
                StatusCode::ACCEPTED,
                &format!("Received selection: {:?}.", selection),
                None,
            )
            .await;

            if let Ok((_, store_path, _)) = crate::db::get_paths(&selection.image_name, &state.pool).await {
                let _ = crate::io::retrieve(&store_path.into(), selection.clone(), sender.clone())
                    .await
                    .map_err(|e| async {
                        log(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            &format!(
                                "Failed to retrieve image with name: {}.",
                                &selection.image_name
                            ),
                            Some(e),
                        )
                        .await;
                    });
            } else {
                log::<String>(
                    StatusCode::BAD_REQUEST,
                    &format!(
                        "Couldn't find image with name: {} in the database.",
                        &selection.image_name
                    ),
                    None,
                )
                .await;
            }
        } else {
            log::<String>(
                StatusCode::BAD_REQUEST,
                &format!("Failed to parse selection: {}.", message),
                None,
            )
            .await;
        }
    }
}