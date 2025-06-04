use crate::api::prelude::*;
use crate::constants::BIN_ID;

#[derive(Deserialize)]
pub struct PathParams {
    pub store_id: u32,
    pub parent_id: u32,
    pub name: String,
}

// TODO: Sanitise directory name.
pub async fn create(
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        parent_id,
        name,
    }): Path<PathParams>,
) -> Response {
    // [CHECK]: Cannot create directory in bin.
    match crate::db::directory::is_within(&dbm, store_id, parent_id, BIN_ID)
        .map(|res| res || parent_id == BIN_ID)
    {
        Ok(false) => {}
        Ok(true) => {
            return logger.error(
                StatusCode::FORBIDDEN,
                Error::RequestIntegrity,
                "DC-E00",
                "Cannot create a directory in the bin.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "DC-E01",
                "Failed to check if new directory would be in the bin.",
                Some(e),
            );
        }
    };

    // [DATABASE]: Insert directory into the database.
    let id = match crate::db::directory::insert(&dbm, store_id, parent_id, &name) {
        Ok(id) => {
            logger.log("Directory inserted into the database.");
            id
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseInsertion,
                "DC-E02",
                "Failed to insert directory into the database.",
                Some(e),
            );
        }
    };

    // [COMMS]: Broadcast directory create message to connected clients.
    match csm
        .broadcast(ServerMsg::Directory(DirectoryServerMsg::Create {
            store_id,
            parent_id,
            id,
            name,
        }))
        .await
    {
        Ok(()) => logger.success(StatusCode::CREATED, "Directory created successfully."),
        Err(e) => logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResponseIntegrity,
            "DC-E03",
            "Failed to encode directory create message.",
            Some(e),
        ),
    }
}
