use crate::api::common::*;

pub async fn delete(Extension(conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete image with id: {id}."),
        None,
    );

    let (name, path) = match crate::db::image::get(id, Arc::clone(&conn)) {
        Ok(paths) => paths,
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to retrieve paths for image with id: {id}."),
                Some(e),
            );
        }
    };

    // Delete image directory from the filesystem.
    let _ = crate::io::delete(&path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Could not delete directory for image with name `{}`.", name),
            Some(e),
        );
    });

    // Remove entries from the state database.
    let _ = crate::db::image::delete(id, Arc::clone(&conn)).map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete image with name `{}` from state database.",
                name
            ),
            Some(e),
        );
    });

    return log::<()>(
        StatusCode::OK,
        &format!("Successfully deleted image with name `{}`.", name),
        None,
    );
}
