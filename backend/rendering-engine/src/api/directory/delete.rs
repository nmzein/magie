use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(id): Path<u32>,
    Query(Params { mode }): Query<Params>,
) -> Response {
    if PRIVILEDGED.contains(&id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DD-E00",
            "Cannot delete priviledged directories.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified directory is not a priviledged directory.",
    );

    if STORES.contains(&id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DD-E01",
            "Invalid way to delete a store, use DELETE /api/stores/:id instead.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified directory is not a store.",
    );

    // Retrieve directory path.
    let directory_path = match crate::db::directory::path(id) {
        Ok(path) => {
            logger.report(
                Check::ResourceExistence,
                "Directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::DatabaseQuery,
                "DD-E02",
                "Failed to retrieve directory path from the database. There is a chance that the directory does not exist.",
                Some(e),
            );
        }
    };

    // Retrieve Bin path.
    let bin_path = match crate::db::directory::path(BIN_ID) {
        Ok(path) => {
            logger.report(
                Check::ResourceExistence,
                "Bin directory exists in the database and its path was successfully retrieved.",
            );

            path
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DD-E03",
                "Bin directory was not found in the database.",
                Some(e),
            );
        }
    };

    if directory_path.starts_with(&bin_path) && mode == DeleteMode::Soft {
        return logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "DD-E04",
            "Cannot soft delete a directory that is already in the Bin.",
            None,
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(&mut logger, id, &directory_path, &bin_path),
        DeleteMode::Hard => hard_delete(&mut logger, id, &directory_path),
    };

    if let Err(response) = result {
        return response;
    }

    let registry = match crate::db::general::get_registry() {
        Ok(registry) => {
            logger.log("Registry retrieved from the database.");

            registry
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DD-E05",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger.success(StatusCode::OK, "Directory deleted successfully.");

    (StatusCode::OK, Json(registry)).into_response()
}

pub fn soft_delete(
    logger: &mut Logger<'_>,
    id: u32,
    directory_path: &std::path::Path,
    bin_path: &std::path::Path,
) -> Result<(), Response> {
    // TODO: Not sure if returning here actually ends this function.
    // Move the directory to the "Bin" in the filesystem.
    match crate::io::r#move(directory_path, bin_path) {
        Ok(()) => {
            logger.log("Directory moved to the Bin in the filesystem.");
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceDeletion,
                "DDS-E00",
                "Failed to soft delete directory from the filesystem.",
                Some(e),
            ));
        }
    }

    // Move the directory to the "Bin" in the database.
    match crate::db::directory::r#move(id, BIN_ID, &MoveMode::SoftDelete) {
        Ok(()) => {
            logger.log("Directory moved to the Bin in the database.");

            Ok(())
        }
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "DDS-E01",
            "Failed to soft delete directory from the database.",
            Some(e),
        )),
    }
}

pub fn hard_delete(
    logger: &mut Logger<'_>,
    id: u32,
    directory_path: &std::path::Path,
) -> Result<(), Response<Body>> {
    // Remove the directory from the filesystem.
    match crate::io::delete(directory_path) {
        Ok(()) => {
            logger.log("Directory deleted from the filesystem.");
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceDeletion,
                "DDH-E00",
                "Failed to hard delete directory from the filesystem.",
                Some(e),
            ));
        }
    };

    // Remove the directory from the database.
    match crate::db::directory::delete(id) {
        Ok(()) => {
            logger.log("Directory deleted from the database.");

            Ok(())
        }
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "DDH-E01",
            "Failed to hard delete directory from the database.",
            Some(e),
        )),
    }
}
