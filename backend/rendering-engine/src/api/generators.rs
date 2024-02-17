use crate::api::common::*;

pub async fn generators(Extension(_state): Extension<AppState>) -> Response {
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    )
    .await;

    // Json(state.generators.keys().cloned().collect::<Vec<_>>()).into_response()
    Json(["TIA Toolbox".to_string()]).into_response()
}