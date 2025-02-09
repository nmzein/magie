use crate::{api::common::*, types::DeleteMode};

#[derive(Deserialize)]
pub struct Params {
    mode: DeleteMode,
}

pub async fn delete(Path(id): Path<u32>, Query(Params { mode }): Query<Params>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "[ID/M00]: Received request to delete image with id `{id}` using mode `{mode:?}`."
        ),
        None,
    );

    let image_path = match crate::db::image::path(id) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[ID/E00]: Image with id `{id}` does not exist in the database."),
                Some(e),
            );
        }
    };

    let bin_path = match crate::db::directory::path(BIN_ID) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                "[ID/E01]: Bin directory was not found in the database.",
                Some(e),
            );
        }
    };

    if image_path.starts_with(&bin_path) && mode == DeleteMode::Soft {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            "[ID/E02]: Cannot soft delete an image that is already in the Bin.",
            None,
        );
    }

    let result = match mode {
        DeleteMode::Soft => soft_delete(id, &image_path, &bin_path).await,
        DeleteMode::Hard => hard_delete(id, &image_path).await,
    };

    if let Err(error) = result {
        return error;
    }

    match crate::db::general::get_registry() {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[ID/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[ID/E03]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}

pub async fn hard_delete(id: u32, image_path: &std::path::Path) -> Result<(), Response> {
    match crate::io::delete(image_path).await {
        Ok(()) => {}
        Err(e) => {
            return Err(log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[ID-H/E00]: Failed to hard delete image with id `{id}` from the filesystem."
                ),
                Some(e),
            ));
        }
    }

    match crate::db::image::delete(id) {
        Ok(()) => Ok(()),
        Err(e) => Err(log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("[ID-H/E01]: Failed to hard delete image with id `{id}` from the database."),
            Some(e),
        )),
    }
}

pub async fn soft_delete(
    id: u32,
    image_path: &std::path::Path,
    bin_path: &std::path::Path,
) -> Result<(), Response> {
    match crate::io::r#move(image_path, bin_path).await {
        Ok(()) => {}
        Err(e) => {
            return Err(log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "[ID-S/E00]: Failed to soft delete image with id `{id}` from the filesystem."
                ),
                Some(e),
            ));
        }
    }

    match crate::db::image::r#move(id, BIN_ID) {
        Ok(()) => Ok(()),
        Err(e) => Err(log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("[ID-S/E01]: Failed to soft delete image with id `{id}` from the database."),
            Some(e),
        )),
    }
}
