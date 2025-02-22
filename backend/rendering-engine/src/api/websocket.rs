use crate::AppState;
use axum::{
    Extension,
    extract::{WebSocketUpgrade, ws::Message},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

// TODO: Have auth layer provide the user id.
const USER_ID: u32 = 0;

pub async fn websocket(
    Extension(state): Extension<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let state = Arc::clone(&state);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        let (sender, mut receiver) = mpsc::channel::<Message>(8);

        // Insert the sender into connections for usage across other endpoints.
        state.add_connection(USER_ID, sender.clone());

        let mut broadcast_receiver = state.broadcast.subscribe();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Send direct messages to user.
                    msg = receiver.recv() => {
                        if let Some(msg) = msg {
                            sink.send(msg).await.ok();
                        } else {
                            break;
                        }
                    }
                    // Send broadcasts to user.
                    Ok(msg) = broadcast_receiver.recv() => {
                        sink.send(msg).await.ok();
                    }
                }
            }

            // Cleanup on disconnect.
            sink.close().await.ok();
        });

        // Handle incoming messages.
        while let Some(Ok(Message::Text(message))) = stream.next().await {
            let state = Arc::clone(&state);

            tokio::spawn(async move {
                // TODO: Send errors to frontend.
                if let Ok(tile) = crate::api::image::tiles::tiles(message) {
                    state.send(USER_ID, tile.into()).await;
                }
            });
        }
    })
}
