use crate::api::common::*;
use shared::types::DeleteMode;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    image_id: u32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    mode: DeleteMode,
}

pub async fn delete(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, image_id }): Path<PathParams>,
    Query(QueryParams { mode }): Query<QueryParams>,
) -> Response {
    // TODO: Again unnecessary?
    let image_path = match crate::db::image::path(store_id, image_id) {
        Ok(path) => path,
        Err(e) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "ID-E00",
                "Image doesn't exist in the database.",
                Some(e),
            );
        }
    };

    let bin_path = match crate::db::stores::bin(store_id) {
        Ok(path) => path,
        Err(e) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "ID-E01",
                "Bin not found in the database.",
                Some(e),
            );
        }
    };

    if image_path.starts_with(&bin_path) && mode == DeleteMode::Soft {
        return logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "ID-E02",
            "Cannot soft delete an image that is already in the Bin.",
            None,
        );
    }

    if let Err(error) = match mode {
        DeleteMode::Soft => soft_delete(&mut logger, store_id, image_id),
        DeleteMode::Hard => hard_delete(&mut logger, store_id, image_id, &image_path),
    } {
        return error;
    };

    (StatusCode::OK).into_response()
}

pub fn soft_delete(logger: &mut Logger<'_>, store_id: u32, image_id: u32) -> Result<(), Response> {
    match crate::db::image::r#move(store_id, image_id, BIN_ID) {
        Ok(()) => Ok(()),
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "IDS-E00",
            "Failed to soft delete image from the database.",
            Some(e),
        )),
    }
}

pub fn hard_delete(
    logger: &mut Logger<'_>,
    store_id: u32,
    image_id: u32,
    image_path: &std::path::Path,
) -> Result<(), Response> {
    match crate::io::delete(image_path) {
        Ok(()) => {}
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceDeletion,
                "IDH-E00",
                "Failed to hard delete image from the filesystem.",
                Some(e),
            ));
        }
    }

    match crate::db::image::delete(store_id, image_id) {
        Ok(()) => Ok(()),
        Err(e) => Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseDeletion,
            "IDH-E01",
            "Failed to hard delete image from the database.",
            Some(e),
        )),
    }
}
