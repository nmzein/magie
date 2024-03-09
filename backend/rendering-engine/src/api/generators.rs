use crate::api::common::*;

pub async fn generators(Extension(AppState { generators, .. }): Extension<AppState>) -> Response {
    #[cfg(feature = "log-success")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    );

    Json(
        generators
            .lock()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
    )
    .into_response()
}
