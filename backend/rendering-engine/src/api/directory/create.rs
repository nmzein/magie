use crate::api::common::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DirectoryCreateRequest {
    pub parent_id: u32,
    pub name: String,
}

pub async fn create(
    Extension(conn): Extension<AppState>,
    Json(DirectoryCreateRequest { parent_id, name }): Json<DirectoryCreateRequest>,
) -> Response {
    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received request to create directory with name `{name}` under parent with id `{parent_id}`."),
        None,
    );

    // Check if a directory with the same name already exists under the parent directory.
    let path = match crate::db::directory::exists(parent_id, &name, Arc::clone(&conn)) {
        Ok(Some(path)) => path,
        Ok(None) => {
            return log::<()>(
                StatusCode::CONFLICT,
                &format!(
                    "Directory with name `{name}` already exists under parent with id `{parent_id}`."
                ),
                None,
            );
        }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to check if directory with name `{name}` exists under parent with id `{parent_id}`."),
                Some(e),
            );
        }
    };

    // Create the directory in the filesystem.
    let _ = crate::io::create(&path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Failed to create directory with name `{name}` under parent with id `{parent_id}`."
            ),
            Some(e),
        );
    });

    // Insert the directory into the database.
    let _ = crate::db::directory::insert(parent_id, &name, Arc::clone(&conn)).map_err(|e|  {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Failed to insert directory with name `{name}` under parent with id `{parent_id}` into the database."
            ),
            Some(e),
        );
    });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "Successfully retrieved registry from the state database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve registry from the state database.",
            Some(e),
        ),
    }
}
