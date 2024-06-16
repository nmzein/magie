use crate::api::common::*;

pub async fn delete(Extension(_conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete directory with id: {id}."),
        None,
    );

    Json(()).into_response()
}
