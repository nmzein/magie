use crate::api::common::*;

pub async fn annotations(
    Extension(AppState { pool, .. }): Extension<AppState>,
    image_name: String,
) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image with name: {}.",
            image_name
        ),
        None,
    );

    let annotations_path = match crate::db::get_paths(&image_name, &pool).await {
        Ok((_, _, Some(annotations_path))) => annotations_path,
        Ok((_, _, None)) => {
            return log_respond::<()>(
                StatusCode::NOT_FOUND,
                &format!("Image with name {} does not have annotations.", image_name),
                None,
            );
        }
        Err(e) => {
            return log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Failed to retrieve annotations path for image with name {}.",
                    image_name,
                ),
                Some(e),
            );
        }
    };

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
