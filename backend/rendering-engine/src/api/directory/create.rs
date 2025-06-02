use crate::api::common::*;

#[derive(Deserialize)]
pub struct PathParams {
    pub store_id: u32,
    pub parent_id: u32,
    pub name: String,
}

// TODO: Sanitise directory name.
pub async fn create(
    Extension(csm): Extension<Arc<ClientSocketManager>>,
    Extension(db): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        parent_id,
        name,
    }): Path<PathParams>,
) -> Response {
    if parent_id == BIN_ID {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DC-E00",
            "Cannot create directory in bin.",
            None,
        );
    } else {
        logger.report(
            Check::RequestIntegrity,
            "Specified parent directory is not bin.",
        );
    }

    // Insert the directory into the database.
    let id = match crate::db::directory::insert(&db, store_id, parent_id, &name) {
        Ok(id) => {
            logger.log("Directory inserted into the database.");
            id
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseInsertion,
                "DC-E01",
                "Failed to insert directory into the database.",
                Some(e),
            );
        }
    };

    // Broadcast directory creation.
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
            "DC-E02",
            "Failed to encode directory create message.",
            Some(e),
        ),
    }
}
