use crate::api::common::*;

// TODO: Retrieve cloud stores.
pub async fn stores(Extension(AppState { pool, .. }): Extension<AppState>) -> Response {
    #[cfg(feature = "log-success")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for list of images.",
        None,
    );

    let Ok(images) = crate::db::list(&pool).await else {
        let resp = log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve list of images.",
            None,
        );

        return resp;
    };

    Json(images).into_response()
}
