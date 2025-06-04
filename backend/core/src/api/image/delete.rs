use crate::api::prelude::*;
use crate::{constants::BIN_ID, types::fs::DeleteMode};

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    image_id: u32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, image_id }): Path<PathParams>,
    Query(QueryParams { mode }): Query<QueryParams>,
) -> Response {
    match mode {
        DeleteMode::Soft => {
            // Need to check if image already in bin or else bad state will happen.
            let parent_id = match crate::db::image::get_parent(&dbm, store_id, image_id) {
                Ok(parent_id) => parent_id,
                Err(e) => {
                    return logger.error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::DatabaseQuery,
                        "IDS-E00",
                        "Failed to get image parent.",
                        Some(e),
                    );
                }
            };

            match crate::db::directory::is_within(&dbm, store_id, parent_id, BIN_ID) {
                Ok(false) => {}
                Ok(true) => {
                    return logger.error(
                        StatusCode::BAD_REQUEST,
                        Error::RequestIntegrity,
                        "IDS-E01",
                        "Cannot soft delete image already in bin.",
                        None,
                    );
                }
                Err(e) => {
                    return logger.error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::DatabaseQuery,
                        "IDS-E02",
                        "Failed to check if image parent is in the bin.",
                        Some(e),
                    );
                }
            };

            // Soft delete the image in the database.
            match crate::db::image::soft_delete(&dbm, store_id, image_id) {
                Ok(()) => logger.success(StatusCode::OK, "Soft deleted image successfully."),
                Err(e) => logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::DatabaseDeletion,
                    "IDS-E03",
                    "Failed to soft delete image from the database.",
                    Some(e),
                ),
            }
        }
        DeleteMode::Hard => {
            match crate::io::delete(store_id, image_id) {
                Ok(()) => {}
                Err(e) => {
                    return logger.error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::ResourceDeletion,
                        "IDH-E00",
                        "Failed to hard delete image from the filesystem.",
                        Some(e),
                    );
                }
            }

            match crate::db::image::delete(&dbm, store_id, image_id) {
                Ok(()) => logger.success(StatusCode::OK, "Hard deleted image successfully."),
                Err(e) => logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::DatabaseDeletion,
                    "IDH-E01",
                    "Failed to hard delete image from the database.",
                    Some(e),
                ),
            }
        }
    }
}
