use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    pub id: u32,
    pub name: String,
}

pub async fn rename(
    Extension(_conn): Extension<AppState>,
    Json(Body { id, name }): Json<Body>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to rename directory with id: {id} to {name}."),
        None,
    );

    Json(()).into_response()
}
