use crate::api::common::*;
use serde::Serialize;

#[derive(Serialize)]
struct AnnotationLayerResponse {
    tag: String,
    visible: bool,
    opacity: f32,
    fill: String,
    stroke: String,
    geometry: String,
}

impl AnnotationLayerResponse {
    fn new(tag: String, geometry: String) -> Self {
        Self {
            tag,
            visible: true,
            opacity: 0.5,
            fill: "#FF0000".to_string(),
            stroke: "#000000".to_string(),
            geometry,
        }
    }
}

pub async fn annotations(Extension(conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request for annotations of image with id: {id}."),
        None,
    );

    let annotation_layer_paths =
        match crate::db::image::get_annotation_layer_paths(id, Arc::clone(&conn)) {
            Ok(layers) => layers,
            Err(e) => {
                return log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Failed to retrieve paths for image with id: {id}."),
                    Some(e),
                );
            }
        };

    let mut annotation_layers = Vec::new();
    for (tag, path) in annotation_layer_paths {
        if let Ok(geometry) = std::fs::read_to_string(&path) {
            annotation_layers.push(AnnotationLayerResponse::new(tag.to_owned(), geometry));
        } else {
            return log::<()>(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to read JSON file at path: {path:?}."),
                None,
            );
        }
    }

    if annotation_layers.is_empty() {
        return log::<()>(
            StatusCode::NOT_FOUND,
            &format!("No annotation layers found for image with id: {id}."),
            None,
        );
    }

    Json(annotation_layers).into_response()
}
