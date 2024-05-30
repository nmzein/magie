use crate::api::common::*;

pub async fn generators() -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    );

    Json(generators::export::names()).into_response()
}
