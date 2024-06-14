use crate::api::common::*;

// TODO: Retrieve cloud stores.
pub async fn registry(Extension(conn): Extension<AppState>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(StatusCode::ACCEPTED, "Received request for registry.", None);

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "Successfully retrieved registry from the state database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve registry from the state database.",
            Some(e),
        ),
    }
}
