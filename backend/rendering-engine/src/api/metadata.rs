use crate::api::common::*;

pub async fn metadata(Extension(state): Extension<AppState>, image_name: String) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for metadata of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    let Ok(metadata) = crate::db::get_metadata(&image_name, &state.pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with name {} does not exist in the database or doesn't have metadata.",
                image_name
            ),
            None,
        )
        .await;
    };

    Json(metadata).into_response()
}