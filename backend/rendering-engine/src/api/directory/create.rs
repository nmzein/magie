use crate::{api::common::*, Logger};

#[derive(Deserialize)]
pub struct Body {
    pub parent_id: u32,
    pub name: String,
}

pub async fn create<'a>(
    Extension(logger): Extension<Arc<Logger>>,
    Extension(conn): Extension<AppState>,
    Json(Body { parent_id, name }): Json<Body>,
) -> Response {
    println!("Got here");

    log::<()>(
        StatusCode::ACCEPTED,
        &format!("[DC/M00]: Received request to create directory with name `{name}` under parent with id `{parent_id}`."),
        None,
    );

    if PRIVILEDGED.contains(&parent_id) {
        return log::<()>(
            StatusCode::FORBIDDEN,
            &format!("[DC/E00]: Cannot create directory under priviledged directories."),
            None,
        );
    }

    // Check if a directory with the same name already exists under the parent directory.
    let path = match crate::db::directory::exists(parent_id, &name, Arc::clone(&conn)) {
        Ok(Some(path)) => path,
        Ok(None) => {
            return log::<()>(
                StatusCode::CONFLICT,
                &format!(
                    "[DC/E01]: Directory with name `{name}` already exists under parent with id `{parent_id}`."
                ),
                None,
            );
        }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[DC/E02]: Failed to check if directory with name `{name}` exists under parent with id `{parent_id}`."),
                Some(e),
            );
        }
    };

    // Create the directory in the filesystem.
    let _ = crate::io::create(&path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DC/E03]: Failed to create directory with name `{name}` under parent with id `{parent_id}`."
            ),
            Some(e),
        );
    });

    // Insert the directory into the database.
    let _ = crate::db::directory::insert(parent_id, &name, Arc::clone(&conn)).map_err(|e|  {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DC/E04]: Failed to insert directory with name `{name}` under parent with id `{parent_id}` into the database."
            ),
            Some(e),
        );
    });

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "[DC/M01]: Successfully retrieved registry from the state database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "[DC/E05]: Failed to retrieve registry from the state database.",
            Some(e),
        ),
    }
}
