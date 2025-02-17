use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    store_id: u32,
    directory_id: u32,
}

#[derive(Deserialize)]
pub struct Body {
    destination_id: u32,
}

pub async fn r#move(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params {
        store_id,
        directory_id,
    }): Path<Params>,
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
    }

    // Check if the directory we're trying to move is priviledged.
    if PRIVILEDGED.contains(&directory_id) {
        return logger.error(
            StatusCode::FORBIDDEN,
            Error::RequestIntegrity,
            "DM-E01",
            "Cannot move priviledged directories.",
            None,
        );
    }

    logger.report(
        Check::RequestIntegrity,
        "Specified parent directory is not a priviledged directory.",
    );

    // Check if the destination is a child of the directory we're trying to move.
    match crate::db::directory::is_within(store_id, destination_id, directory_id) {
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
    match crate::db::directory::r#move(store_id, directory_id, destination_id, &MoveMode::Regular) {
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

    logger.success(StatusCode::OK, "Directory moved successfully.");

    (StatusCode::OK).into_response()
}
