use crate::api::common::*;

pub async fn registry(Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>) -> Response {
    match crate::db::general::get_registry() {
        Ok(registry) => {
            logger
                .lock()
                .unwrap()
                .success(StatusCode::OK, "Retrieved registry.");

            return Json(registry).into_response();
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQueryError,
                "RG-E00",
                "Failed to retrieve registry.",
                Some(e),
            );
        }
    }
}
