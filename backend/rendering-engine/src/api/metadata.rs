use crate::api::common::*;

pub async fn metadata(Extension(conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request for metadata of image with id: {id}."),
        None,
    );

    // Get image with id from the database.
    let Ok(image) = crate::db::get(id, Arc::clone(&conn)).await else {
        return log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Image with id: {id} does not exist in the state database.",),
            None,
        );
    };

    #[cfg(feature = "log.success")]
    log::<()>(
        StatusCode::OK,
        &format!("Successfully fetched metadata for image with id: {id}."),
        None,
    );

    Json(image.metadata_layers).into_response()
}
