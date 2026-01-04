use crate::api::prelude::*;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    asset_id: u32,
}

pub async fn properties(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, asset_id }): Path<PathParams>,
) -> Response {
    match crate::db::image::properties(&dbm, store_id, asset_id) {
        Ok(properties) => {
            logger.success(StatusCode::OK, "Retrieved asset properties successfully.");
            Json(properties).into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "AP-E00",
                "Failed to retrieve asset properties.",
                Some(e),
            );
        }
    }
}
