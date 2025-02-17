use crate::api::common::*;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    directory_id: u32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        directory_id,
    }): Path<PathParams>,
    Query(QueryParams { mode }): Query<QueryParams>,
) -> Response {
    if PRIVILEDGED.contains(&directory_id) {
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

    // TODO: Unnecessary?
    let inside_bin = match crate::db::directory::is_within(store_id, directory_id, BIN_ID) {
        Ok(b) => b,
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DD-E01",
                "Failed to check if directory is in the Bin.",
                Some(e),
            );
        }
    };

    if mode == DeleteMode::Soft && inside_bin {
        return logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "DD-E02",
            "Cannot soft delete a directory that is already in the Bin.",
            None,
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(&mut logger, store_id, directory_id),
        DeleteMode::Hard => hard_delete(&mut logger, store_id, directory_id),
    };

    if let Err(response) = result {
        return response;
    }

    logger.success(StatusCode::OK, "Directory deleted successfully.");

    (StatusCode::OK).into_response()
}

pub fn soft_delete(
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response> {
    // Move the directory to the "Bin" in the database.
    match crate::db::directory::r#move(store_id, directory_id, BIN_ID, &MoveMode::SoftDelete) {
        Ok(()) => {
            logger.log("Directory moved to the Bin in the database.");
            Ok(())
        }
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "DDS-E00",
            "Failed to soft delete directory from the database.",
            Some(e),
        )),
    }
}

// TODO: Actually hard delete images by retrieving them from the database and deleting them.
pub fn hard_delete(
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response<Body>> {
    // Remove the directory from the database.
    let images = match crate::db::stores::get_images_below(store_id, directory_id) {
        Ok(images) => {
            logger.log("Directory deleted from the database.");
            images
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseDeletion,
                "DDH-E00",
                "Failed to hard delete directory from the database.",
                Some(e),
            ))
        }
    };

    for image in images {
        match crate::io::delete(store_id, image.id) {
            Ok(()) => {}
            Err(e) => {
                return Err(logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::DatabaseDeletion,
                    "DDH-E01",
                    "Failed to hard delete image from the database.",
                    Some(e),
                ))
            }
        }
    }

    logger.log("Child images deleted from the filesystem.");

    // Remove the directory from the database.
    match crate::db::directory::delete(store_id, directory_id) {
        Ok(()) => {
            logger.log("Directory deleted from the database.");
            Ok(())
        }
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "DDH-E00",
            "Failed to hard delete directory from the database.",
            Some(e),
        )),
    }
}
