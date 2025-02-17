use crate::api::common::*;

#[derive(Deserialize)]
pub struct Params {
    store_id: u32,
}

pub async fn get(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { store_id }): Path<Params>,
) -> Response {
    match crate::db::stores::get(store_id) {
        Ok(store) => Json(store).into_response(),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "SG-E00",
                "Failed to retrieve store.",
                Some(e),
            );
        }
    }
}
