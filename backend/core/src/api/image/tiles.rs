use crate::types::{
    database::DatabaseManager,
    messages::{TileClientMsg, TileServerMsg},
};

// TODO: Capture large rectangles of selections rather than individual tiles.
// TODO: Cache in an in-memory HashMap?
pub fn tiles(
    dbm: &DatabaseManager,
    TileClientMsg {
        store_id,
        id,
        level,
        x,
        y,
    }: TileClientMsg,
) -> Result<TileServerMsg, String> {
    let path = match crate::db::image::image_path(dbm, store_id, id) {
        Ok(path) => path,
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
