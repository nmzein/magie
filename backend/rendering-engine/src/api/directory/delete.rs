use crate::api::common::*;
use crate::types::MoveMode;
use axum::extract::Path;
use std::path::PathBuf;

static BIN_ID: u32 = 1;

pub async fn delete(Extension(conn): Extension<AppState>, Path(id): Path<u32>) -> Response {
    // Retrieve directory path.
    let directory_path = match crate::db::directory::path(id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[DD/E00]: Directory with id `{id}` does not exist in the database."),
                Some(e),
            );
        }
    };

    // Retrieve Bin path.
    let bin_path = match crate::db::directory::path(BIN_ID, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[DD/E01]: Bin directory was not found in the database."),
                Some(e),
            );
        }
    };

    let mut in_bin: bool = false;

    for ancestor in directory_path.ancestors() {
        if ancestor == bin_path {
            in_bin = true;
        }

        if in_bin {
            break;
        }
    }

    if in_bin {
        hard_delete(id, &directory_path, Arc::clone(&conn)).await
    } else {
        soft_delete(id, &directory_path, &bin_path, Arc::clone(&conn)).await
    }
}

pub async fn hard_delete(id: u32, directory_path: &PathBuf, conn: AppState) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[DD-H/M00]: Received request to hard delete directory with id `{id}`."),
        None,
    );

    // Remove the directory from the filesystem.
    let _ = crate::io::delete(&directory_path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DD-H/E00]: Failed to hard delete directory with id `{id}` from the filesystem."
            ),
            Some(e),
        );
    });

    // Remove the directory from the database.
    let _ = crate::db::directory::delete(id, Arc::clone(&conn)).map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DD-H/E01]: Failed to hard delete directory with id `{id}` from the database."
            ),
            Some(e),
        );
    });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[DD-H/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[DD-H/E02]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}

pub async fn soft_delete(
    id: u32,
    directory_path: &PathBuf,
    bin_path: &PathBuf,
    conn: AppState,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[DD-S/M00]: Received request to soft delete directory with id `{id}`."),
        None,
    );

    // Move the directory to the "Bin".
    let _ = crate::io::r#move(&directory_path, &bin_path)
        .await
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[DD-S/E00]: Failed to soft delete directory with id `{id}` from the filesystem."
                ),
                Some(e),
            );
        });

    // Move the directory in the database.
    let _ = crate::db::directory::r#move(id, BIN_ID, MoveMode::SoftDelete, Arc::clone(&conn))
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[DD-S/E01]: Failed to soft delete directory with id `{id}` from the database."
                ),
                Some(e),
            );
        });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[DD-S/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[DD-S/E02]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}
