use crate::api::common::*;

pub async fn metadata(
    Extension(AppState {
        pool,
        current_image,
        ..
    }): Extension<AppState>,
    image_name: String,
) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for metadata of image with name: {}.",
            image_name
        ),
        None,
    );

    let Ok(image) = crate::db::get(&image_name, &pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with name {} does not exist in the database or doesn't have metadata.",
                image_name
            ),
            None,
        );
    };

    *current_image.lock().unwrap() = Some(image.clone());

    Json(image.metadata).into_response()
}
