use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    parent_id: u32,
}

pub async fn r#move(
    Extension(conn): Extension<AppState>,
    Path(id): Path<u32>,
    Json(Body { parent_id }): Json<Body>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[IM/M00]: Received request to move image with id `{id}` to directory with id `{parent_id}`."),
        None,
    );

    if parent_id == ROOT_ID {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            &format!("[IM/E00]: Cannot move images into the root directory."),
            None,
        );
    }

    // Retrieve target image path.
    let target_image_path = match crate::db::image::path(id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[IM/E01]: Target image with id `{id}` does not exist in the database."),
                Some(e),
            );
        }
    };

    // Retrieve destination directory path.
    let dest_directory_path = match crate::db::directory::path(parent_id, Arc::clone(&conn)) {
        Ok(path) => path,
        Err(e) => {
            return log(
                StatusCode::NOT_FOUND,
                &format!("[IM/E02]: Destination directory with id `{parent_id}` does not exist in the database."),
                Some(e),
            );
        }
    };

    // Move the directory in the filesystem.
    let _ = crate::io::r#move(&target_image_path, &dest_directory_path)
        .await
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("[IM/E03]: Failed to move image with id `{id}` into directory with id `{parent_id}` in the filesystem."),
                Some(e),
            );
        });

    // Move the image in the database.
    let _ = crate::db::image::r#move(id, parent_id, Arc::clone(&conn))
        .map_err(|e| {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IM/E04]: Failed to move image with id `{id}` into directory with id `{parent_id}` in the database."),

                Some(e),
            );
        });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[IM/M01]: Successfully retrieved registry from the database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[IM/E05]: Failed to retrieve registry from the database.",
            Some(e),
        ),
    }
}
