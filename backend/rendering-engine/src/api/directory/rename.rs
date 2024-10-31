use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    name: String,
}

pub async fn rename(
    Extension(_conn): Extension<AppState>,
    Path(id): Path<u32>,
    Json(Body { name }): Json<Body>,
) -> Response {
    Json(()).into_response()
}
