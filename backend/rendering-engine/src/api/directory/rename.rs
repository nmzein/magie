use crate::api::common::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryRenameRequest {
    pub id: u32,
    pub name: String,
}

pub async fn rename(
    Extension(_conn): Extension<AppState>,
    Json(DirectoryRenameRequest { id, name }): Json<DirectoryRenameRequest>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to rename directory with id: {id} to {name}."),
        None,
    );

    Json(()).into_response()
}
