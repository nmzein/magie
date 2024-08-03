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
                &format!("DD1: Directory with id `{id}` does not exist."),
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
                &format!("DD2: Bin directory does not exist."),
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
        // Hard delete
        todo!();
    } else {
        soft_delete(Arc::clone(&conn), id, &directory_path, &bin_path).await
    }
}

pub async fn soft_delete(
    conn: AppState,
    id: u32,
    directory_path: &PathBuf,
    bin_path: &PathBuf,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to soft delete directory with id: {id}."),
        None,
    );

    // Move the directory to the "Bin".
    let _ = crate::io::r#move(&directory_path, &bin_path)
        .await
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "DD3: Failed to soft delete directory with id `{id}` from the filesystem."
                ),
                Some(e),
            );
        });

    // Move the directory in the database.
    let _ = crate::db::directory::r#move(id, BIN_ID, MoveMode::SoftDelete, Arc::clone(&conn))
        .map_err(|e| async {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("DD4: Failed to soft delete directory with id `{id}` from the database."),
                Some(e),
            );
        });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "Successfully retrieved registry from the state database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve registry from the state database.",
            Some(e),
        ),
    }
}
