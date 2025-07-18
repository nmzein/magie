use crate::api::prelude::*;

pub async fn generators(Extension(mut logger): Extension<Logger<'_>>) -> Response {
    let generators = generators::export::names();

    logger.success(StatusCode::OK, "Retrieved annotation generators.");

    Json(generators).into_response()
}
