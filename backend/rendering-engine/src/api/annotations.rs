use crate::api::common::*;

pub async fn annotations(
    Extension(AppState { current_image, .. }): Extension<AppState>,
) -> Response {
    let Some(current_image) = current_image.lock().unwrap().clone() else {
        return log_respond::<()>(
            StatusCode::BAD_REQUEST,
            "Image metadata must first be fetched before requesting tiles.",
            None,
        );
    };

    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image: {:?}.",
            current_image.image_name
        ),
        None,
    );

    let Some(ref annotations_name) = current_image.annotations_name else {
        return log_respond::<()>(
            StatusCode::NOT_FOUND,
            &format!(
                "Image with name {} does not have annotations.",
                current_image.image_name
            ),
            None,
        );
    };

    let annotations_path = current_image.directory_path.join(annotations_name);
    let Ok(annotations) = crate::io::annotations(&annotations_path).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve annotations.",
            None,
        );
    };

    #[cfg(feature = "log")]
    log::<()>(StatusCode::OK, "Successfully retrieved annotations.", None);

    Json(annotations).into_response()
}
