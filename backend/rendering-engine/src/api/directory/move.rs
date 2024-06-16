use crate::api::common::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryMoveRequest {
    pub id: u32,
    pub parent_id: u32,
}

pub async fn r#move(
    Extension(_conn): Extension<AppState>,
    Json(DirectoryMoveRequest { id, parent_id }): Json<DirectoryMoveRequest>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to move directory with id: {id} to parent with id {parent_id}."),
        None,
    );

    Json(()).into_response()
}
