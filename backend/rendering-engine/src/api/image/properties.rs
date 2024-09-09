use crate::api::common::*;

pub async fn properties(Extension(conn): Extension<AppState>, Path(id): Path<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[IP/M00]: Received request for properties of image with id: {id}."),
        None,
    );

    // Get image properties from the database.
    let properties = match crate::db::image::properties(id, Arc::clone(&conn)) {
        Ok(properties) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                &format!("[IP/M01]: Successfully fetched properties for image with id `{id}`."),
                None,
            );

            properties
        }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[IP/E00]: Could not fetch properties for image with id `{id}`."),
                Some(e),
            );
        }
    };

    Json(properties).into_response()
}
