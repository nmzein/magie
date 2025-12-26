use crate::api::prelude::*;
use crate::types::{
    messages::{ClientMsg, AssetServerMsg, TileClientMsg, TileServerMsg},
    user::User,
};
use axum::extract::{WebSocketUpgrade, ws::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

#[derive(Deserialize)]
pub struct Params {
    store_id: u32,
    asset_id: u32,
}

pub async fn websocket(
    Extension(user): Extension<User>,
    Extension(db): Extension<Arc<DatabaseManager>>,
    // Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { store_id, asset_id }): Path<Params>,
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let csm = Arc::clone(&csm);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        let (sender, mut receiver) = mpsc::channel::<Message>(8);

        // Insert the sender into connections for usage across other endpoints.
        csm.add_asset_connection(user.id, store_id, asset_id, sender.clone());

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
                        csm.remove_asset_connection(user.id, store_id, asset_id);
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
                        match get_tile(&db, store_id, asset_id, tile_request) {
                            Ok(tile_response) => {
                                let _ = csm.send_asset(user.id, store_id, asset_id, AssetServerMsg::Tile(tile_response)).await;
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
                                let _ = csm.send_asset(user.id, store_id, asset_id, AssetServerMsg::Error(e)).await;
                            }
                        }
                    }
                }
            });
        }
    })
}


// TODO: Capture large rectangles of selections rather than individual tiles.
pub fn get_tile(
    dbm: &DatabaseManager,
    store_id: u32,
    asset_id: u32,
    TileClientMsg {
        level,
        x,
        y,
    }: TileClientMsg,
) -> Result<TileServerMsg, String> {
    let path = match crate::db::image::image_path(dbm, store_id, asset_id) {
        Ok(path) => path,
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve path for image with id: {asset_id}. {e}");
            return Err(format!(
                "WebSocket Error: Failed to retrieve path for image with id: {asset_id}. {e}"
            ));
        }
    };

    let tile = match crate::io::retrieve(&path, level, x, y) {
        Ok(tile) => tile,
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve tile for image with id: {asset_id}. {e}");
            return Err(format!(
                "WebSocket Error: Failed to retrieve tile for image with id: {asset_id}. {e}"
            ));
        }
    };

    Ok(tile)
}
