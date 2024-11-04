use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    parent_id: u32,
}

pub async fn r#move(
    Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>,
    Extension(conn): Extension<AppState>,
    Path(id): Path<u32>,
    Json(Body { parent_id }): Json<Body>,
) -> Response {
    if PRIVILEDGED.contains(&id) {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DM-E00",
            "Cannot move priviledged directories.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified parent directory is not a priviledged directory.",
    );

    if STORES.contains(&id) {
        logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DM-E01",
            "Cannot move stores.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified directory is not a store.",
    );

    if parent_id == ROOT_ID {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DM-E02",
            "Cannot move directories into the root directory.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified parent directory is not the root directory.",
    );

    // Retrieve target directory path.
    let target_directory_path = match crate::db::directory::path(id, Arc::clone(&conn)) {
        Ok(path) => {
            logger.lock().unwrap().report(
                Check::ResourceExistenceCheck,
                "Target directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQueryError,
                "DM-E03",
                "Target directory does not exist in the database.",
                Some(e),
            );
        }
    };

    // Retrieve destination directory path.
    let dest_directory_path = match crate::db::directory::path(parent_id, Arc::clone(&conn)) {
        Ok(path) => {
            logger.lock().unwrap().report(
                Check::ResourceExistenceCheck,
                "Destination directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQueryError,
                "DM-E04",
                "Destination directory does not exist in the database.",
                Some(e),
            );
        }
    };

    // Check destination is not inside target.
    if dest_directory_path.starts_with(&target_directory_path) {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DM-E05",
            "Cannot move directory into itself.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Destination directory is not inside target directory.",
    );

    // Move the directory in the filesystem.
    let _ = crate::io::r#move(&target_directory_path, &dest_directory_path)
        .await
        .map_err(|e| {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMoveError,
                "DM-E06",
                "Failed to move directory in the filesystem.",
                Some(e),
            );
        });

    logger
        .lock()
        .unwrap()
        .log("Directory moved in the filesystem.");

    // Move the directory in the database.
    let _ = crate::db::directory::r#move(id, parent_id, MoveMode::Regular, Arc::clone(&conn))
        .map_err(|e| {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMoveError,
                "DM-E07",
                "Failed to move directory in the database.",
                Some(e),
            );
        });

    logger
        .lock()
        .unwrap()
        .log("Directory moved in the database.");

    let registry = match crate::db::general::get_registry(Arc::clone(&conn)) {
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
                "DM-E08",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger
        .lock()
        .unwrap()
        .success(StatusCode::OK, "Directory moved successfully.");

    return (StatusCode::OK, Json(registry)).into_response();
}
