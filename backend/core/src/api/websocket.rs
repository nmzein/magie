use crate::api::prelude::*;
use crate::types::user::User;
use axum::extract::{WebSocketUpgrade, ws::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub async fn websocket(
    Extension(user): Extension<User>,
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let csm = Arc::clone(&csm);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        let (sender, mut receiver) = mpsc::channel::<Message>(8);

        // Insert the sender into connections for usage across other endpoints.
        csm.add_general_connection(user.id, sender.clone());

        let mut broadcast_receiver = csm.broadcast.subscribe();

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
        while let Some(message) = stream.next().await {
            let csm = Arc::clone(&csm);

            tokio::spawn(async move {
                match message {
                    Ok(Message::Binary(_)) => return,
                    Ok(Message::Text(_)) => return,
                    Ok(Message::Ping(_)) => return,
                    Ok(Message::Pong(_)) => return,
                    _ => {
                        csm.remove_all_connections(user.id);
                        return;
                    }
                };
            });
        }
    })
}
