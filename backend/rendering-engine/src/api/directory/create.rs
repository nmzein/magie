use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    pub store_id: u32,
    pub parent_id: u32,
    pub name: String,
}

// TODO: Sanitise directory name
pub async fn create(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params {
        store_id,
        parent_id,
        name,
    }): Path<Params>,
) -> Response {
    if parent_id == BIN_ID {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DC-E00",
            "Cannot create directory in Bin.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified parent directory is not Bin.",
    );

    // Insert the directory into the database.
    match crate::db::directory::insert(store_id, parent_id, &name) {
        Ok(()) => logger.log("Directory inserted into the database."),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseInsertion,
                "DC-E01",
                "Failed to insert directory into the database.",
                Some(e),
            );
        }
    }

    logger.success(StatusCode::CREATED, "Directory created successfully.");

    (StatusCode::CREATED).into_response()
}
