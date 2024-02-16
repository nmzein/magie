use crate::api::common::*;

pub async fn metadata(Extension(state): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for metadata of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    match crate::db::get_metadata(&image_name, &state.pool).await {
        Ok(metadata) => Json(metadata).into_response(),
        Err(e) => {
            log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve metadata.",
                Some(e),
            )
            .await
        }
    }
}