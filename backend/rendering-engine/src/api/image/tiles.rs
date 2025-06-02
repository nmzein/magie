use crate::types::{
    database::DatabaseManager,
    messages::{TileClientMsg, TileServerMsg},
};

// TODO: Capture large rectangles of selections rather than individual tiles.
pub fn tiles(
    db: &DatabaseManager,
    TileClientMsg {
        store_id,
        id,
        level,
        x,
        y,
    }: TileClientMsg,
) -> Result<TileServerMsg, String> {
    // TODO: Cache in an in-memory HashMap.
    // TODO: No hard coding.
    let path = match crate::db::image::path(db, store_id, id) {
        Ok(path) => path.join("image.zarr"),
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve path for image with id: {id}. {e}");
            return Err(format!(
                "WebSocket Error: Failed to retrieve path for image with id: {id}. {e}"
            ));
        }
    };

    let tile = match crate::io::retrieve(&path, level, x, y) {
        Ok(tile) => tile,
        Err(e) => {
            println!("WebSocket Error: Failed to retrieve tile for image with id: {id}. {e}");
            return Err(format!(
                "WebSocket Error: Failed to retrieve tile for image with id: {id}. {e}"
            ));
        }
    };

    Ok(tile)
}
