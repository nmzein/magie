use crate::api::common::*;

pub async fn annotations(Extension(state): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    let Ok((_, _, Some(annotations_path))) = crate::db::get_paths(&image_name, &state.pool).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with name {} does not exist in the database or doesn't have annotations.",
                image_name
            ),
            None,
        )
        .await;
    };

    let Ok(annotations) = crate::io::annotations(&annotations_path).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve annotations.",
            None,
        )
        .await;
    };
    
    log::<String>(StatusCode::OK, "Successfully retrieved annotations.", None).await;

    Json(annotations).into_response()
}
