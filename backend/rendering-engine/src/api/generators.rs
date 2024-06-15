use crate::api::common::*;

pub async fn generators() -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    );

    let generators = generators::export::names();

    #[cfg(feature = "log.success")]
    log(
        StatusCode::OK,
        "Successfully retrieved annotation generators.",
        Some(&generators),
    );

    Json(generators).into_response()
}
