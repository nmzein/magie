use crate::api::common::*;

pub async fn registry(Extension(mut logger): Extension<Logger<'_>>) -> Response {
    match crate::db::registry::get() {
        Ok(registry) => {
            logger.success(StatusCode::OK, "Retrieved registry.");
            Json(registry).into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "RG-E00",
                "Failed to retrieve registry.",
                Some(e),
            );
        }
    }
}
