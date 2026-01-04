use crate::api::prelude::*;
use axum::{body::Bytes, http::header};

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    asset_id: u32,
    annotation_layer_id: u32,
}

pub async fn annotations(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        asset_id,
        annotation_layer_id,
    }): Path<PathParams>,
) -> Response {
    let path =
        match crate::db::image::annotation_path(&dbm, store_id, asset_id, annotation_layer_id) {
            Ok(path) => path,
            Err(e) => {
                return logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::DatabaseQuery,
                    "IA-E00",
                    "Failed to retrieve path of annotation layer from registry database.",
                    Some(e),
                );
            }
        };

    // Read the binary content of the GLB file.
    match std::fs::read(&path) {
        Ok(file_data) => {
            logger.success(StatusCode::OK, "Annotation layer retrieved successfully.");

            (
                axum::response::AppendHeaders([
                    (header::CONTENT_TYPE, "application/octet-stream"),
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename=\"a{annotation_layer_id}.drc\""),
                    ),
                ]),
                Bytes::from(file_data),
            )
                .into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceExistence,
                "IA-E01",
                &format!(
                    "Failed to retrieve annotation layer file: {}",
                    path.display()
                ),
                Some(e.into()),
            );
        }
    }
}
