use crate::api::prelude::*;
use crate::constants::BIN_ID;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    asset_id: u32,
}

#[derive(Deserialize)]
pub struct Body {
    destination_id: u32,
}

pub async fn r#move(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, asset_id }): Path<PathParams>,
    Json(Body { destination_id }): Json<Body>,
) -> Response {
    // [CHECK]: Cannot move asset into bin.
    match crate::db::directory::is_or_in(&dbm, store_id, destination_id, BIN_ID) {
        Ok(false) => {}
        Ok(true) => {
            return logger.error(
                StatusCode::FORBIDDEN,
                Error::RequestIntegrity,
                "AM-E00",
                "Cannot move an asset into the bin.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "AM-E01",
                "Failed to check if asset destination would be in the bin.",
                Some(e),
            );
        }
    };

    match crate::db::image::r#move(&dbm, store_id, asset_id, destination_id) {
        Ok(()) => logger.success(StatusCode::OK, "Moved asset successfully."),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "AM-E02",
                "Failed to move asset in the database.",
                Some(e),
            );
        }
    }
}
