use crate::api::common::*;

pub async fn delete(Extension(conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete image with id: {id}."),
        None,
    );

    let paths = match crate::db::get_paths(id, Arc::clone(&conn)).await {
        Ok(paths) => paths,
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to retrieve paths for image with id: {id}."),
                Some(e),
            );
        }
    };

    // Delete directory from the filesystem.
    let _ = crate::io::delete(&paths.directory_path)
        .await
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Could not delete directory for image with name `{}`.",
                    paths.image_name
                ),
                Some(e),
            );
        });

    // Remove entries from the state database.
    let _ = crate::db::remove(id, Arc::clone(&conn))
        .await
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Could not delete image with name `{}` from state database.",
                    paths.image_name
                ),
                Some(e),
            );
        });

    return log::<()>(
        StatusCode::OK,
        &format!(
            "Successfully deleted image with name `{}`.",
            paths.image_name
        ),
        None,
    );
}
