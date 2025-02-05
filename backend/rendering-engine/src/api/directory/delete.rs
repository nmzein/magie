use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>,
    Path(id): Path<u32>,
    Query(Params { mode }): Query<Params>,
) -> Response {
    if PRIVILEDGED.contains(&id) {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DD-E00",
            "Cannot delete priviledged directories.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified directory is not a priviledged directory.",
    );

    if STORES.contains(&id) {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DD-E01",
            "Invalid way to delete a store, use DELETE /api/stores/:id instead.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified directory is not a store.",
    );

    // Retrieve directory path.
    let directory_path = match crate::db::directory::path(id) {
        Ok(path) => {
            logger.lock().unwrap().report(
                Check::ResourceExistenceCheck,
                "Directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQueryError,
                "DD-E02",
                "Failed to retrieve directory path from the database. There is a chance that the directory does not exist.",
                Some(e),
            );
        }
    };

    // Retrieve Bin path.
    let bin_path = match crate::db::directory::path(BIN_ID) {
        Ok(path) => {
            logger.lock().unwrap().report(
                Check::ResourceExistenceCheck,
                "Bin directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQueryError,
                "DD-E03",
                "Bin directory was not found in the database.",
                Some(e),
            );
        }
    };

    if directory_path.starts_with(&bin_path) && mode == DeleteMode::Soft {
        return logger.lock().unwrap().error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrityError,
            "DD-E04",
            "Cannot soft delete a directory that is already in the Bin.",
            None,
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(Arc::clone(&logger), id, &directory_path, &bin_path).await,
        DeleteMode::Hard => hard_delete(Arc::clone(&logger), id, &directory_path).await,
    };

    if let Err(response) = result {
        return response;
    }

    let registry = match crate::db::general::get_registry() {
        Ok(registry) => {
            logger
                .lock()
                .unwrap()
                .log("Registry retrieved from the database.");

            registry
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQueryError,
                "DD-E05",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger
        .lock()
        .unwrap()
        .success(StatusCode::OK, "Directory deleted successfully.");

    return (StatusCode::OK, Json(registry)).into_response();
}

pub async fn soft_delete(
    logger: Arc<Mutex<Logger<'_>>>,
    id: u32,
    directory_path: &PathBuf,
    bin_path: &PathBuf,
) -> Result<(), Response> {
    // TODO: Not sure if returning here actually ends this function.
    // Move the directory to the "Bin" in the filesystem.
    let _ = crate::io::r#move(&directory_path, &bin_path)
        .await
        .map_err(|e| {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceDeletionError,
                "DDS-E00",
                "Failed to soft delete directory from the filesystem.",
                Some(e),
            );
        })?;

    logger
        .lock()
        .unwrap()
        .log("Directory moved to the Bin in the filesystem.");

    // Move the directory to the "Bin" in the database.
    let _ = crate::db::directory::r#move(id, BIN_ID, MoveMode::SoftDelete).map_err(|e| {
        return logger.lock().unwrap().error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletionError,
            "DDS-E01",
            "Failed to soft delete directory from the database.",
            Some(e),
        );
    })?;

    logger
        .lock()
        .unwrap()
        .log("Directory moved to the Bin in the database.");

    Ok(())
}

pub async fn hard_delete(
    logger: Arc<Mutex<Logger<'_>>>,
    id: u32,
    directory_path: &PathBuf,
) -> Result<(), Response<Body>> {
    // Remove the directory from the filesystem.
    let _ = crate::io::delete(&directory_path).await.map_err(|e| {
        return Err::<(), Response<Body>>(logger.lock().unwrap().error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResourceDeletionError,
            "DDH-E00",
            "Failed to hard delete directory from the filesystem.",
            Some(e),
        ));
    });

    logger
        .lock()
        .unwrap()
        .log("Directory deleted from the filesystem.");

    // Remove the directory from the database.
    let _ = crate::db::directory::delete(id).map_err(|e| {
        return Err::<(), Response<Body>>(logger.lock().unwrap().error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletionError,
            "DDH-E01",
            "Failed to hard delete directory from the database.",
            Some(e),
        ));
    });

    logger
        .lock()
        .unwrap()
        .log("Directory deleted from the database.");

    Ok(())
}
