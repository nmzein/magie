use crate::api::prelude::*;
use crate::types::{
    messages::{ClientMsg, ServerMsg},
    user::User,
};
use axum::extract::{WebSocketUpgrade, ws::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub async fn websocket(
    Extension(user): Extension<User>,
    Extension(db): Extension<Arc<DatabaseManager>>,
    // Extension(mut logger): Extension<Logger<'_>>,
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let csm = Arc::clone(&csm);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        let (sender, mut receiver) = mpsc::channel::<Message>(8);

        // Insert the sender into connections for usage across other endpoints.
        csm.add_connection(user.id, sender.clone());

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
            let db = Arc::clone(&db);

            tokio::spawn(async move {
                let message = match message {
                    Ok(Message::Binary(message)) => message,
                    Ok(Message::Text(_)) => return,
                    Ok(Message::Ping(_)) => return,
                    Ok(Message::Pong(_)) => return,
                    _ => {
                        csm.remove_connection(user.id);
                        //logger.success(StatusCode::OK, "Client disconnected");
                        return;
                    }
                };

                let message = match ClientMsg::try_from(message) {
                    Ok(message) => message,
                    Err(_) => {
                        // logger.error(
                        //     StatusCode::BAD_REQUEST,
                        //     Error::WebSocketParse,
                        //     "WS-E00",
                        //     "Failed to parse client message.",
                        //     Some(e.into()),
                        // );
                        return;
                    }
                };

                match message {
                    ClientMsg::Tile(tile_request) => {
                        match crate::api::image::tiles::tiles(&db, tile_request) {
                            Ok(tile_response) => {
                                let _ = csm.send(user.id, ServerMsg::Tile(tile_response)).await;
                                // else {
                                //     logger.error(
                                //         StatusCode::INTERNAL_SERVER_ERROR,
                                //         Error::WebSocketSend,
                                //         "WS-E01",
                                //         "Failed to send message.",
                                //         None,
                                //     );
                                //     return;
                                // };
                            }
                            Err(e) => {
                                let _ = csm.send(user.id, ServerMsg::Error(e)).await;
                            }
                        }
                    }
                }
            });
        }
    })
}
