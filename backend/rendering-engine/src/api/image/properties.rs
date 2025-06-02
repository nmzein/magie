use crate::api::common::*;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    image_id: u32,
}

pub async fn properties(
    Extension(db): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, image_id }): Path<PathParams>,
) -> Response {
    match crate::db::image::properties(&db, store_id, image_id) {
        Ok(properties) => {
            logger.success(StatusCode::OK, "Retrieved asset properties successfully.");
            Json(properties).into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IP-E00",
                "Failed to retrieve asset properties.",
                Some(e),
            );
        }
    }
}
