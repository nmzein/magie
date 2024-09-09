use crate::api::common::*;

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

    let annotations = match std::fs::read_to_string(&path) {
        Ok(annotations) => annotations,
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IA/E01]: Failed to read JSON file for annotation layer with id `{annotation_layer_id}` at path: `{path:?}`."),
                Some(e),
            );
        }
    };

    Json(annotations).into_response()
}
