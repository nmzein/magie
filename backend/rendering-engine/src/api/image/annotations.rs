use crate::api::common::*;
use axum::{body::Bytes, http::header};

#[derive(Deserialize)]
pub struct Params {
    image_id: u32,
    annotation_layer_id: u32,
}

pub async fn annotations(
    Extension(conn): Extension<AppState>,
    Query(Params {
        image_id,
        annotation_layer_id,
    }): Query<Params>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[IA/M00]: Received request for annotation layer with id `{annotation_layer_id}` of image with id: `{image_id}`."),
        None,
    );

    let path = match crate::db::image::get_annotation_layer_path(
        image_id,
        annotation_layer_id,
        Arc::clone(&conn),
    ) {
        Ok(layers) => layers,
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IA/E00]: Failed to retrieve path of annotation layer with id `{annotation_layer_id}` of image with id: `{image_id}`."),
                Some(e),
            );
        }
    };

    // Read the binary content of the GLB file
    match std::fs::read(&path) {
        Ok(file_data) => (
            axum::response::AppendHeaders([
                (header::CONTENT_TYPE, "model/gltf-binary"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"file.glb\"",
                ),
            ]),
            Bytes::from(file_data),
        )
            .into_response(),
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IA/E01]: Failed to read GLB file for annotation layer with id `{annotation_layer_id}` at path: `{path:?}`."),
                Some(e),
            );
        }
    }
}
