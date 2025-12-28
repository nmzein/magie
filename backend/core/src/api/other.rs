use crate::api::prelude::*;

pub async fn registry(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
) -> Response {
    match crate::db::registry::get(&dbm) {
        Ok(registry) => {
            logger.success(StatusCode::OK, "Retrieved registry.");
            Json(registry).into_response()
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "RG-E00",
                "Failed to retrieve registry.",
                Some(e),
            );
        }
    }
}

#[derive(Serialize)]
struct Modules<'a> {
    generators: Vec<&'a str>,
    decoders: Vec<&'a str>,
    encoders: Vec<&'a str>,
}

pub async fn modules(Extension(mut logger): Extension<Logger<'_>>) -> Response {
    let generators = generators::export::names();
    let decoders = decoders::export::names();
    let encoders = encoders::export::names();

    logger.success(StatusCode::OK, "Retrieved modules.");

    Json(Modules {
        generators,
        decoders,
        encoders,
    }).into_response()
}
