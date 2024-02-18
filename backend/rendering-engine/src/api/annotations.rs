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

    let Ok((_, _, Some(annotations_path))) = crate::db::get_paths(&image_name, &pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with name {} does not exist in the database or doesn't have annotations.",
                image_name
            ),
            None,
        );
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
