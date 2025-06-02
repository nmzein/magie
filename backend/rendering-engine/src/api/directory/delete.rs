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
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    Extension(db): Extension<Arc<DatabaseManager>>,
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
    } else {
        logger.report(
            Check::RequestIntegrity,
            "Specified directory is not a priviledged directory.",
        );
    }

    let inside_bin = match crate::db::directory::is_within(&db, store_id, directory_id, BIN_ID) {
        Ok(b) => b,
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DD-E01",
                "Failed to check if directory is in the bin.",
                Some(e),
            );
        }
    };

    if mode == DeleteMode::Soft && inside_bin {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DD-E02",
            "Cannot soft delete a directory that is already in the bin.",
            None,
        );
    } else {
        logger.report(
            Check::RequestIntegrity,
            "Not soft deleting a directory that is already in the bin.",
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(&db, &mut logger, store_id, directory_id),
        DeleteMode::Hard => hard_delete(&db, &mut logger, store_id, directory_id),
    };

    if let Err(response) = result {
        return response;
    }

    // Broadcast directory move if soft delete.
    if mode == DeleteMode::Soft {
        match csm
            .broadcast(ServerMsg::Directory(DirectoryServerMsg::Move {
                store_id,
                id: directory_id,
                destination_id: BIN_ID,
            }))
            .await
        {
            Ok(()) => logger.success(StatusCode::OK, "Directory soft deleted successfully."),
            Err(e) => logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResponseIntegrity,
                "DD-E03",
                "Failed to encode directory move message.",
                Some(e),
            ),
        }
    } else {
        // Broadcast directory deletion.
        match csm
            .broadcast(ServerMsg::Directory(DirectoryServerMsg::Delete {
                store_id,
                id: directory_id,
            }))
            .await
        {
            Ok(()) => logger.success(StatusCode::OK, "Directory hard deleted successfully."),
            Err(e) => logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResponseIntegrity,
                "DD-E04",
                "Failed to encode directory delete message.",
                Some(e),
            ),
        }
    }
}

pub fn soft_delete(
    db: &DatabaseManager,
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response> {
    // Move the directory to the bin in the database.
    match crate::db::directory::r#move(&db, store_id, directory_id, BIN_ID, &MoveMode::SoftDelete) {
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

pub fn hard_delete(
    db: &DatabaseManager,
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response<Body>> {
    // Fetch child images from the database.
    let images = match crate::db::stores::get_images_below(db, store_id, directory_id) {
        Ok(images) => {
            logger.log("Child images fetched from database.");
            images
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DDH-E00",
                "Failed to fetch child images from the database.",
                Some(e),
            ));
        }
    };

    for image in images {
        match crate::io::delete(store_id, image.id) {
            Ok(()) => {}
            Err(e) => {
                return Err(logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::ResourceDeletion,
                    "DDH-E01",
                    "Failed to hard delete image from the filesystem.",
                    Some(e),
                ));
            }
        }
    }

    logger.log("Child images deleted from the filesystem.");

    // Remove the directory from the database.
    match crate::db::directory::delete(&db, store_id, directory_id) {
        Ok(()) => {
            logger.log("Directory deleted from the database.");
            Ok(())
        }
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "DDH-E02",
            "Failed to hard delete directory from the database.",
            Some(e),
        )),
    }
}
