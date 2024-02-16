use crate::api::common::*;

// TODO: Retrieve cloud stores.
pub async fn stores(Extension(state): Extension<AppState>) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for list of images.",
        None,
    )
    .await;

    if let Ok(images) = crate::db::list(&state.pool).await {
        return Json(images).into_response();
    }

    log_respond::<String>(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to retrieve list of images.",
        None,
    )
    .await
}
