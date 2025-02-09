use crate::api::common::*;

pub async fn properties(
    Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>,
    Path(id): Path<u32>,
) -> Response {
    // Get image properties from the database.
    match crate::db::image::properties(id) {
        Ok(properties) => {
            logger
                .lock()
                .unwrap()
                .success(StatusCode::OK, "Retrieved asset properties successfully.");

            Json(properties).into_response()
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IP-E00",
                "Failed to retrieve asset properties.",
                Some(e),
            );
        }
    }
}
