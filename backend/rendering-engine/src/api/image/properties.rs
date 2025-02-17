use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    store_id: u32,
    image_id: u32,
}

pub async fn properties(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { store_id, image_id }): Path<Params>,
) -> Response {
    match crate::db::image::properties(store_id, image_id) {
        Ok(properties) => {
            logger.success(StatusCode::OK, "Retrieved asset properties successfully.");
            Json(properties).into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IP-E00",
                "Failed to retrieve asset properties.",
                Some(e),
            );
        }
    }
}
