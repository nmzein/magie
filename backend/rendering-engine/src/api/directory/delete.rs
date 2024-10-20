use crate::{
    api::common::*,
    types::{DeleteMode, MoveMode},
};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Params {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(conn): Extension<AppState>,
    Path(id): Path<u32>,
    Query(Params { mode }): Query<Params>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "[DD/M00]: Received request to delete directory with id `{id}` using mode `{mode:?}`."
        ),
        None,
    );

    if PRIVILEDGED.contains(&id) {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            &format!("[DD/E00]: Cannot delete priviledged directories."),
            None,
        );
    }

    if STORES.contains(&id) {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            &format!("[DD/E01]: Invalid way to delete stores."),
            None,
        );
    }

    // Retrieve directory path.
    let directory_path = match crate::db::directory::path(id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[DD/E02]: Directory with id `{id}` does not exist in the database."),
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
                &format!("[DD/E03]: Bin directory was not found in the database."),
                Some(e),
            );
        }
    };

    if directory_path.starts_with(&bin_path) && mode == DeleteMode::Soft {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            &format!("[DD/E04]: Cannot soft delete a directory that is already in the Bin."),
            None,
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(id, &directory_path, &bin_path, Arc::clone(&conn)).await,
        DeleteMode::Hard => hard_delete(id, &directory_path, Arc::clone(&conn)).await,
    };

    if let Err(error) = result {
        return error;
    }

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[DD/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[DD/E05]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}

pub async fn hard_delete(
    id: u32,
    directory_path: &PathBuf,
    conn: AppState,
) -> Result<(), Response> {
    // Remove the directory from the filesystem.
    let _ = crate::io::delete(&directory_path).await.map_err(|e| {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DD-H/E00]: Failed to hard delete directory with id `{id}` from the filesystem."
            ),
            Some(e),
        );
    });

    // Remove the directory from the database.
    let _ = crate::db::directory::delete(id, Arc::clone(&conn)).map_err(|e| {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DD-H/E01]: Failed to hard delete directory with id `{id}` from the database."
            ),
            Some(e),
        );
    });

    Ok(())
}

pub async fn soft_delete(
    id: u32,
    directory_path: &PathBuf,
    bin_path: &PathBuf,
    conn: AppState,
) -> Result<(), Response> {
    // Move the directory to the "Bin" in the filesystem.
    let _ = crate::io::r#move(&directory_path, &bin_path)
        .await
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[DD-S/E00]: Failed to soft delete directory with id `{id}` from the filesystem."
                ),
                Some(e),
            );
        });

    // Move the directory to the "Bin" in the database.
    let _ = crate::db::directory::r#move(id, BIN_ID, MoveMode::SoftDelete, Arc::clone(&conn))
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[DD-S/E01]: Failed to soft delete directory with id `{id}` from the database."
                ),
                Some(e),
            );
        });

    Ok(())
}
