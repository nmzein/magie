use crate::api::common::*;
use crate::constants::PRIVILEGED;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    directory_id: u32,
}

#[derive(Deserialize)]
pub struct Body {
    destination_id: u32,
}

pub async fn r#move(
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        directory_id,
    }): Path<PathParams>,
    Json(Body { destination_id }): Json<Body>,
) -> Response {
    if directory_id == destination_id {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E00",
            "Cannot move directory into itself.",
            None,
        );
    } else {
        logger.report(Check::RequestIntegrity, "Not moving directory into itself.");
    }

    // Check if the directory we're trying to move is privileged.
    if PRIVILEGED.contains(&directory_id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E01",
            "Cannot move privileged directories.",
            None,
        );
    } else {
        logger.report(
            Check::RequestIntegrity,
            "Specified parent directory is not a privileged directory.",
        );
    }

    // Check if the destination is a child of the directory we're trying to move.
    match crate::db::directory::is_within(&dbm, store_id, destination_id, directory_id) {
        Ok(false) => {
            logger.report(
                Check::RequestIntegrity,
                "Destination directory is not inside target directory.",
            );
        }
        Ok(true) => {
            return logger.error(
                StatusCode::FORBIDDEN,
                Error::RequestIntegrity,
                "DM-E02",
                "Cannot move directory into its children.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DM-E03",
                "Failed to check if attempting to move directory inside its children.",
                Some(e),
            );
        }
    };

    // Move the directory in the database.
    match crate::db::directory::r#move(&dbm, store_id, directory_id, destination_id) {
        Ok(()) => logger.log("Directory moved in the database."),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "DM-E04",
                "Failed to move directory in the database.",
                Some(e),
            );
        }
    }

    // Broadcast directory move.
    match csm
        .broadcast(ServerMsg::Directory(DirectoryServerMsg::Move {
            store_id,
            id: directory_id,
            destination_id,
        }))
        .await
    {
        Ok(()) => logger.success(StatusCode::OK, "Directory moved successfully."),
        Err(e) => logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResponseIntegrity,
            "DM-E05",
            "Failed to encode directory move message.",
            Some(e),
        ),
    }
}
