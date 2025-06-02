use crate::api::common::*;
use crate::constants::{BIN_ID, PRIVILEGED};

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
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        directory_id,
    }): Path<PathParams>,
    Query(QueryParams { mode }): Query<QueryParams>,
) -> Response {
    // [CHECK]: Cannot delete privileged directories (root and bin).
    if PRIVILEGED.contains(&directory_id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DD-E00",
            "Cannot delete privileged directories.",
            None,
        );
    }

    // [CHECK]: Cannot soft delete within the bin.
    if mode == DeleteMode::Soft {
        match crate::db::directory::is_within(&dbm, store_id, directory_id, BIN_ID) {
            Ok(false) => {}
            Ok(true) => {
                return logger.error(
                    StatusCode::FORBIDDEN,
                    Error::RequestIntegrity,
                    "DD-E01",
                    "Cannot soft delete a directory that is already in the bin.",
                    None,
                );
            }
            Err(e) => {
                return logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::DatabaseQuery,
                    "DD-E02",
                    "Failed to check if directory is in the bin.",
                    Some(e),
                );
            }
        };
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(&dbm, &mut logger, store_id, directory_id),
        DeleteMode::Hard => hard_delete(&dbm, &mut logger, store_id, directory_id),
    };

    if let Err(response) = result {
        return response;
    }

    let message = match mode {
        DeleteMode::Soft => DirectoryServerMsg::Move {
            store_id,
            id: directory_id,
            destination_id: BIN_ID,
        },
        DeleteMode::Hard => DirectoryServerMsg::Delete {
            store_id,
            id: directory_id,
        },
    };

    // [COMMS]: Broadcast to connected clients.
    match csm.broadcast(ServerMsg::Directory(message)).await {
        Ok(()) => logger.success(StatusCode::OK, "Directory deleted successfully."),
        Err(e) => logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResponseIntegrity,
            "DD-E03",
            "Failed to encode directory message.",
            Some(e),
        ),
    }
}

pub fn soft_delete(
    dbm: &DatabaseManager,
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response> {
    // [DATABASE]: Move the directory to the bin in the database.
    match crate::db::directory::soft_delete(dbm, store_id, directory_id) {
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
    dbm: &DatabaseManager,
    logger: &mut Logger<'_>,
    store_id: u32,
    directory_id: u32,
) -> Result<(), Response<Body>> {
    // Fetch child images from the database.
    let images = match crate::db::stores::get_images_below(dbm, store_id, directory_id) {
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

    // Delete the images from the filesystem.
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
    match crate::db::directory::delete(&dbm, store_id, directory_id) {
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
