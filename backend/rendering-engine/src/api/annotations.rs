use crate::api::common::*;

pub async fn annotations(
    Extension(AppState { current_image, .. }): Extension<AppState>,
) -> Response {
    let Some(current_image) = current_image.lock().unwrap().clone() else {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            "Image metadata must first be fetched before requesting tiles.",
            None,
        );
    };

    #[cfg(feature = "log-success")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image: {:?}.",
            current_image.image_name
        ),
        None,
    );

    let Some(ref annotations_name) = current_image.annotations_name else {
        let resp = log::<()>(
            StatusCode::NOT_FOUND,
            &format!(
                "Image with name {} does not have annotations.",
                current_image.image_name
            ),
            None,
        );

        return resp;
    };

    let annotations_path = current_image.directory_path.join(annotations_name);
    let annotations = match crate::io::annotations(&annotations_path).await {
        Ok(annotations) => {
            #[cfg(feature = "log-success")]
            log::<()>(StatusCode::OK, "Successfully retrieved annotations.", None);

            annotations
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve annotations.",
                Some(e),
            );

            return resp;
        }
    };

    Json(annotations).into_response()
}
