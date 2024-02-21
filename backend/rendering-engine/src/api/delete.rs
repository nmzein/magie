use crate::api::common::*;

pub async fn delete(
    Extension(AppState {
        pool,
        current_image,
        ..
    }): Extension<AppState>,
    Json(id): Json<u32>,
) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete image with id: {}.", id),
        None,
    );

    let Ok((directory_path, image_name, _, _)) = crate::db::get_paths(id, &pool).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Could not retrieve paths for image with id {}.", id),
            None,
        );
    };

    // Delete directory from the filesystem.
    let _ = crate::io::delete(&directory_path).await.map_err(|e| async {
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
    let _ = crate::db::remove(id, &pool).await.map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete image with name {} from state database.",
                image_name
            ),
            Some(e),
        );
    });

    // If image in in-memory state, remove its entry.
    let mut binding = current_image.lock().unwrap();

    if let Some(current_image_unwrapped) = binding.clone() {
        if current_image_unwrapped.directory_path == directory_path {
            *binding = None;
        }
    }

    log_respond::<()>(StatusCode::OK, "Successfully deleted image entry.", None)
}
