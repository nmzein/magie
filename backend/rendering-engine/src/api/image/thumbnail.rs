use crate::api::common::*;
use axum::{body::Bytes, http::header::CONTENT_TYPE};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn thumbnail(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(id): Path<u32>,
) -> Response {
    let path = match crate::db::image::get(id) {
        Ok((_, path)) => path.join("thumbnail.jpeg"),
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
    match File::open(&path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            // Read the file content into a buffer.
            match file.read_to_end(&mut buffer).await {
                Ok(_) => {
                    logger.success(StatusCode::OK, "Retrieved asset thumbnail successfully.");

                    // Create a response with the binary content of the image.
                    (
                        StatusCode::OK,
                        [(CONTENT_TYPE, "image/jpeg")],
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
                "IT-E01",
                "Failed to open asset thumbnail.",
                Some(e.into()),
            );
        }
    }
}
