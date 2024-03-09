use crate::api::common::*;
use crate::structs::Paths;

pub async fn delete(
    Extension(AppState {
        conn,
        current_image,
        ..
    }): Extension<AppState>,
    Json(id): Json<u32>,
) -> Response {
    #[cfg(feature = "log-success")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete image with id: {}.", id),
        None,
    );

    let Ok(Paths {
        directory_path,
        image_name,
        ..
    }) = crate::db::get_paths(id, Arc::clone(&conn)).await
    else {
        let resp = log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Could not retrieve paths for image with id {}.", id),
            None,
        );

        return resp;
    };

    // Delete directory from the filesystem.
    let _ = crate::io::delete(&directory_path).await.map_err(|e| async {
        let resp = log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete directory for image with name {}.",
                image_name
            ),
            Some(e),
        );

        return resp;
    });

    // Remove entries from the state database.
    let _ = crate::db::remove(id, Arc::clone(&conn))
        .await
        .map_err(|e| async {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Could not delete image with name {} from state database.",
                    image_name
                ),
                Some(e),
            );

            return resp;
        });

    // If image in in-memory state, remove its entry.
    let mut binding = current_image.lock().unwrap();

    if let Some(current_image_unwrapped) = binding.clone() {
        if current_image_unwrapped.directory_path == directory_path {
            *binding = None;
        }
    }

    let resp = log::<()>(StatusCode::OK, "Successfully deleted image entry.", None);

    resp
}
