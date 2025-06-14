use crate::api::prelude::*;
use axum::{
    body::Bytes,
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
};
use std::{fs::File, io::Read};

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    image_id: u32,
}

pub async fn thumbnail(
    Extension(db): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, image_id }): Path<PathParams>,
) -> Response {
    let path = match crate::db::image::thumbnail_path(&db, store_id, image_id) {
        Ok(path) => path,
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IT-E00",
                "Failed to retrieve asset thumbnail path.",
                Some(e),
            );
        }
    };

    // Try to open and read the thumbnail image.
    match File::open(&path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            // Read the file content into a buffer.
            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    logger.success(StatusCode::OK, "Retrieved asset thumbnail successfully.");

                    // Create a response with the binary content of the image.
                    (
                        StatusCode::OK,
                        [
                            (CONTENT_TYPE, "image/jpeg"),
                            (CACHE_CONTROL, "public, max-age=86400"),
                        ],
                        Bytes::from(buffer),
                    )
                        .into_response()
                }
                Err(e) => {
                    return logger.error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::ResourceRead,
                        "IT-E01",
                        "Failed to read asset thumbnail.",
                        Some(e.into()),
                    );
                }
            }
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceRead,
                "IT-E02",
                "Failed to open asset thumbnail.",
                Some(e.into()),
            );
        }
    }
}
