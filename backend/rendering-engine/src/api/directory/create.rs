use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    pub parent_id: u32,
    pub name: String,
}

pub async fn create(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { parent_id, name }): Path<Params>,
) -> Response {
    if PRIVILEDGED.contains(&parent_id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DC-E00",
            "Cannot create directory under priviledged directories.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified parent directory is not a priviledged directory.",
    );

    // Check if a directory with the same name already exists under the parent directory.
    let path = match crate::db::directory::exists(parent_id, &name) {
        Ok(Some(path)) => {
            logger.report(
                Check::ResourceConflict,
                "Directory name is unique under parent.",
            );

            path
        }
        Ok(None) => {
            return logger.error(
                StatusCode::CONFLICT,
                Error::ResourceConflict,
                "DC-E01",
                "Directory with the same name exists.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DC-E02",
                "Failed to check if directory with name exists under parent.",
                Some(e),
            );
        }
    };

    // Create the directory in the filesystem.
    match crate::io::create(&path) {
        Ok(()) => {
            logger.log("Directory created in the filesystem.");
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "DC-E03",
                "Failed to create directory.",
                Some(e),
            );
        }
    }

    // Insert the directory into the database.
    match crate::db::directory::insert(parent_id, &name) {
        Ok(()) => {
            logger.log("Directory inserted into the database.");
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseInsertion,
                "DC-E04",
                "Failed to insert directory into the database.",
                Some(e),
            );
        }
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
                "DC-E05",
                "Failed to retrieve registry from the database.",
                Some(e),
            )
        }
    };

    logger.success(StatusCode::CREATED, "Directory created successfully.");

    (StatusCode::CREATED, Json(registry)).into_response()
}
