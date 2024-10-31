use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    pub parent_id: u32,
    pub name: String,
}

pub async fn create(
    Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>,
    Extension(conn): Extension<AppState>,
    Path(Params { parent_id, name }): Path<Params>,
) -> Response {
    if PRIVILEDGED.contains(&parent_id) {
        return logger.lock().unwrap().error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrityError,
            "DC-E00",
            "Cannot create directory under priviledged directories.",
            None,
        );
    }

    logger.lock().unwrap().report(
        Check::RequestIntegrityCheck,
        "Specified parent directory is not a priviledged directory.",
    );

    // Check if a directory with the same name already exists under the parent directory.
    let path = match crate::db::directory::exists(parent_id, &name, Arc::clone(&conn)) {
        Ok(Some(path)) => {
            logger.lock().unwrap().report(
                Check::ResourceConflictCheck,
                "Directory name is unique under parent.",
            );

            path
        }
        Ok(None) => {
            return logger.lock().unwrap().error(
                StatusCode::CONFLICT,
                Error::ResourceConflictError,
                "DC-E01",
                "Directory with the same name exists.",
                None,
            );
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQueryError,
                "DC-E02",
                "Failed to check if directory with name exists under parent.",
                Some(e),
            );
        }
    };

    // Create the directory in the filesystem.
    let _ = crate::io::create(&path).await.map_err(|e| {
        return logger.lock().unwrap().error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResourceCreationError,
            "DC-E03",
            "Failed to create directory.",
            Some(e),
        );
    });

    logger
        .lock()
        .unwrap()
        .log("Directory created in the filesystem.");

    // Insert the directory into the database.
    let _ = crate::db::directory::insert(parent_id, &name, Arc::clone(&conn)).map_err(|e| {
        return logger.lock().unwrap().error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseInsertionError,
            "DC-E04",
            "Failed to insert directory into the database.",
            Some(e),
        );
    });

    logger
        .lock()
        .unwrap()
        .log("Directory inserted into the database.");

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
                "DC-E05",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger
        .lock()
        .unwrap()
        .success(StatusCode::CREATED, "Directory created successfully.");

    return (StatusCode::CREATED, Json(registry)).into_response();
}
