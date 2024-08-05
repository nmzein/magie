use crate::{api::common::*, types::MoveMode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryMoveRequest {
    pub target_id: u32,
    pub dest_id: u32,
}

pub async fn r#move(
    Extension(conn): Extension<AppState>,
    Json(DirectoryMoveRequest { target_id, dest_id }): Json<DirectoryMoveRequest>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[DM/M00]: Received request to move directory with id `{target_id}` to directory with id `{dest_id}`."),
        None,
    );

    // Retrieve target directory path.
    let target_directory_path = match crate::db::directory::path(target_id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!(
                    "[DM/E00]: Target directory with id `{target_id}` does not exist in the database."
                ),
                Some(e),
            );
        }
    };

    // Retrieve destination directory path.
    let dest_directory_path = match crate::db::directory::path(dest_id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[DM/E01]: Destination directory with id `{dest_id}` does not exist in the database."),
                Some(e),
            );
        }
    };

    // Move the directory in the filesystem.
    let _ = crate::io::r#move(&target_directory_path, &dest_directory_path)
        .await
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("[DM/E02]: Failed to move directory with id `{target_id}` to directory with id `{dest_id}` in the filesystem."),
                Some(e),
            );
        });

    // Move the directory in the database.
    let _ = crate::db::directory::r#move(target_id, dest_id, MoveMode::Regular, Arc::clone(&conn))
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[DM/E03]: Failed to move directory with id `{target_id}` to directory with id `{dest_id}` in the database."),

                Some(e),
            );
        });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[DM/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[DM/E04]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}
