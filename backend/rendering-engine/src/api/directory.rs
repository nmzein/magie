use crate::api::common::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryCreateRequest {
    pub parent_id: u32,
    pub name: String,
}

pub async fn create(
    Extension(_conn): Extension<AppState>,
    Json(DirectoryCreateRequest { parent_id, name }): Json<DirectoryCreateRequest>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to create directory with name: {name} under parent with id {parent_id}."),
        None,
    );

    Json(()).into_response()
}

pub async fn delete(Extension(_conn): Extension<AppState>, Json(id): Json<u32>) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete directory with id: {id}."),
        None,
    );

    Json(()).into_response()
}

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
