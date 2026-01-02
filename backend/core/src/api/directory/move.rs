use crate::api::prelude::*;
use crate::constants::{BIN_ID, PRIVILEGED};

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
    // [CHECK]: Cannot move privileged directories.
    if PRIVILEGED.contains(&directory_id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E00",
            "Cannot move privileged directories.",
            None,
        );
    }

    // [CHECK]: Cannot move a directory into itself or its children.
    match crate::db::directory::is_or_in(&dbm, store_id, destination_id, directory_id) {
        Ok(false) => {}
        Ok(true) => {
            return logger.error(
                StatusCode::FORBIDDEN,
                Error::RequestIntegrity,
                "DM-E01",
                "Cannot move directory into its children.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DM-E02",
                "Failed to check if attempting to move directory inside its children.",
                Some(e),
            );
        }
    }

    // [CHECK]: Cannot move directory into bin.
    match crate::db::directory::is_or_in(&dbm, store_id, directory_id, BIN_ID) {
        Ok(false) => {}
        Ok(true) => {
            return logger.error(
                StatusCode::FORBIDDEN,
                Error::RequestIntegrity,
                "DM-E03",
                "Cannot create a directory in the bin.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DM-E04",
                "Failed to check if new directory would be in the bin.",
                Some(e),
            );
        }
    };

    // Move the directory in the database.
    match crate::db::directory::r#move(&dbm, store_id, directory_id, destination_id) {
        Ok(()) => {}
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "DM-E05",
                "Failed to move directory in the database.",
                Some(e),
            );
        }
    }

    logger.log("Directory moved in the database.");

    match csm
        .broadcast(GeneralServerMsg::Directory(DirectoryServerMsg::Move {
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
            "DM-E06",
            "Failed to encode directory move message.",
            Some(e),
        ),
    }
}
