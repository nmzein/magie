use crate::api::common::*;

// TODO: Retrieve cloud stores.
pub async fn stores(Extension(conn): Extension<AppState>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for store hierarchy.",
        None,
    );

    let Ok(images) = crate::db::list(Arc::clone(&conn)).await else {
        return log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve store hierarchy from the state database.",
            None,
        );
    };

    #[cfg(feature = "log.success")]
    log::<()>(
        StatusCode::OK,
        "Successfully retrieved store hierarchy from the state database.",
        None,
    );

    Json(images).into_response()
}
