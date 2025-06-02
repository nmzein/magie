use crate::api::common::*;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    image_id: u32,
}

#[derive(Deserialize)]
pub struct Body {
    destination_id: u32,
}

pub async fn r#move(
    Extension(db): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams { store_id, image_id }): Path<PathParams>,
    Json(Body { destination_id }): Json<Body>,
) -> Response {
    match crate::db::image::r#move(&db, store_id, image_id, destination_id) {
        Ok(()) => logger.success(StatusCode::OK, "Moved asset successfully."),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceMove,
                "IM-E00",
                "Failed to move image in the database.",
                Some(e),
            );
        }
    }
}
