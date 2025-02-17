use axum::extract::ws::{Message, Utf8Bytes};
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug, Deserialize)]
pub struct TileRequest {
    pub store_id: u32,
    pub image_id: u32,
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

// TODO: Send error messages to frontend.
// TODO: Capture large rectangles of selections rather than individual tiles.
pub async fn tiles(message: Utf8Bytes, sender: Sender<Message>) {
    // TODO: Move to custom binary message format.
    let TileRequest {
        store_id,
        image_id,
        level,
        x,
        y,
    } = match serde_json::from_str::<TileRequest>(&message) {
        Ok(tile_request) => tile_request,
        Err(e) => {
            println!("WebSocket Error: Failed to parse tile request: {message}. {e}",);
            return;
        }
    };

    // TODO: Cache in an in-memory HashMap.
    let path = match crate::db::image::path(store_id, image_id) {
        Ok(path) => path.join("image.zarr"),
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve path for image with id: {image_id}. {e}",);
            return;
        }
    };

    let tile = match crate::io::retrieve(&path, level, x, y) {
        Ok(tile) => tile,
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve tile for image with id: {image_id}. {e}",);
            return;
        }
    };

    let _ = sender
        .send(Message::Binary(tile.into()))
        .await
        .map_err(|e| {
            println!("WebSocket Error: Failed to send tile for image with id: {image_id}. {e}",);
        });
}
