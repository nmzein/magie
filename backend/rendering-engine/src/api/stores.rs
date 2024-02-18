use crate::api::common::*;

// TODO: Retrieve cloud stores.
pub async fn stores(Extension(AppState { pool, .. }): Extension<AppState>) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for list of images.",
        None,
    );

    let Ok(images) = crate::db::list(&pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve list of images.",
            None,
        );
    };

    Json(images).into_response()
}
