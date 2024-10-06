use crate::api::common::*;
use axum::{body::Bytes, http::header::CONTENT_TYPE};
use tokio::{fs::File, io::AsyncReadExt};

pub async fn thumbnail(Extension(conn): Extension<AppState>, Path(id): Path<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[IT/M00]: Received request for thumbnail of image with id `{id}`."),
        None,
    );

    let mut path = match crate::db::image::get(id, Arc::clone(&conn)) {
        Ok((_, path)) => path,
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IT/E00]: Failed to retrieve thumbnail of image with id `{id}`."),
                Some(e),
            );
        }
    };

    path = path.join("thumbnail.jpg");

    // Try to open and read the thumbnail image
    match File::open(&path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            // Read the file content into a buffer
            match file.read_to_end(&mut buffer).await {
                Ok(_) => {
                    // Create a response with the binary content of the image
                    (
                        StatusCode::OK,
                        [(CONTENT_TYPE, "image/jpeg")],
                        Bytes::from(buffer),
                    )
                        .into_response()
                }
                Err(e) => {
                    // Error reading the file
                    log(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("[IT/E01]: Failed to read the thumbnail of image with id `{id}`."),
                        Some(e),
                    )
                }
            }
        }
        Err(e) => {
            // Error opening the file
            log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IT/E02]: Failed to open the thumbnail of image with id `{id}`."),
                Some(e),
            )
        }
    }
}
