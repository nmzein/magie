use crate::api::common::*;
use crate::types::TileRequest;
use axum::extract::ws::{Message, Utf8Bytes};
use tokio::sync::mpsc::Sender;

// TODO: Send error messages to frontend.
// TODO: Capture large rectangles of selections rather than individual tiles.
pub async fn tiles(message: Utf8Bytes, sender: Sender<Message>) {
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
    let (_, path) = match crate::db::image::get(tile_request.id) {
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
    let encoded_img_path = path.join("image.zarr");
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

    let _ = sender
        .send(Message::Binary(tile.into()))
        .await
        .map_err(|e| {
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
}
