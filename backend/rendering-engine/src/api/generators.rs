use crate::api::common::*;

pub async fn generators(Extension(logger): Extension<Arc<Mutex<Logger<'_>>>>) -> Response {
    let generators = generators::export::names();

    logger
        .lock()
        .unwrap()
        .success(StatusCode::OK, "Retrieved annotation generators.");

    return Json(generators).into_response();
}
