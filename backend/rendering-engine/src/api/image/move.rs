use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    store_id: u32,
    image_id: u32,
}

#[derive(Deserialize)]
pub struct Body {
    destination_id: u32,
}

pub async fn r#move(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { store_id, image_id }): Path<Params>,
    Json(Body { destination_id }): Json<Body>,
) -> Response {
    match crate::db::image::r#move(store_id, image_id, destination_id) {
        Ok(()) => {
            logger.success(StatusCode::OK, "Moved asset successfully.");
            (StatusCode::OK).into_response()
        }
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
