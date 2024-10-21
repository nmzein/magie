use std::time::Instant;

use crate::{api::common::*, Logger};

#[derive(Deserialize)]
pub struct Params {
    pub name: String,
    pub parent: u32,
}

pub async fn create(
    Extension(logger): Extension<Arc<Logger>>,
    Extension(conn): Extension<AppState>,
    Query(Params { name, parent }): Query<Params>,
) -> Response {
    if PRIVILEDGED.contains(&parent) {
        return log::<()>(
            StatusCode::FORBIDDEN,
            &format!("[DC/E00]: Cannot create directory under priviledged directories."),
            None,
        );
    }

    let start = Instant::now();

    // Check if a directory with the same name already exists under the parent directory.
    let path = match crate::db::directory::exists(parent, &name, Arc::clone(&conn)) {
        Ok(Some(path)) => path,
        Ok(None) => {
            return log::<()>(
                StatusCode::CONFLICT,
                &format!(
                    "[DC/E01]: Directory with name `{name}` already exists under parent with id `{parent}`."
                ),
                None,
            );
        }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("[DC/E02]: Failed to check if directory with name `{name}` exists under parent with id `{parent}`."),
                Some(e),
            );
        }
    };

    // logger.message("Conflict Check", start.elapsed().as_millis(), "");

    // Create the directory in the filesystem.
    let _ = crate::io::create(&path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DC/E03]: Failed to create directory with name `{name}` under parent with id `{parent}`."
            ),
            Some(e),
        );
    });

    // Insert the directory into the database.
    let _ = crate::db::directory::insert(parent, &name, Arc::clone(&conn)).map_err(|e|  {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "[DC/E04]: Failed to insert directory with name `{name}` under parent with id `{parent}` into the database."
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
