use crate::api::common::*;

pub async fn delete(
    Extension(AppState { pool, .. }): Extension<AppState>,
    image_name: String,
) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request to delete image with name: {}.",
            image_name
        ),
        None,
    );

    // Delete directory from the filesystem.
    let _ = crate::io::delete(&image_name).await.map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete directory for image with name {}.",
                image_name
            ),
            Some(e),
        );
    });

    // Remove entries from the state database.
    let _ = crate::db::remove(&image_name, &pool)
        .await
        .map_err(|e| async {
            return log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Could not delete image with name {} from state database.",
                    image_name
                ),
                Some(e),
            );
        });

    log_respond::<()>(StatusCode::OK, "Successfully deleted image entry.", None)
}
