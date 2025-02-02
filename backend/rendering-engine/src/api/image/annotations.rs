use crate::api::common::*;
use axum::{body::Bytes, http::header};

#[derive(Deserialize)]
pub struct Params {
    image_id: u32,
    annotation_layer_id: u32,
}

pub async fn annotations(
    Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>,
    Path(Params {
        image_id,
        annotation_layer_id,
    }): Path<Params>,
) -> Response {
    let path = match crate::db::image::get_annotation_layer_path(image_id, annotation_layer_id) {
        Ok(layers) => layers,
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQueryError,
                "IA-E00",
                "Failed to retrieve path of annotation layer from registry database.",
                Some(e),
            );
        }
    };

    // Read the binary content of the GLB file
    let response = match std::fs::read(&path) {
        Ok(file_data) => {
            logger
                .lock()
                .unwrap()
                .log("GLB annotation layer file retrieved from filesystem.");

            (
                axum::response::AppendHeaders([
                    (header::CONTENT_TYPE, "model/gltf-binary"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"file.glb\"",
                    ),
                ]),
                Bytes::from(file_data),
            )
                .into_response()
        }
        Err(e) => {
            return logger.lock().unwrap().error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceExistenceError,
                "IA-E01",
                "Failed to retrieve GLB annotation layer file.",
                Some(e.into()),
            )
        }
    };

    logger
        .lock()
        .unwrap()
        .success(StatusCode::OK, "Annotation layer retrieved successfully.");

    return response;
}
