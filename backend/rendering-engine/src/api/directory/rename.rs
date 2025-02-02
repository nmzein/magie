use crate::api::common::*;

#[derive(Deserialize)]
pub struct Body {
    name: String,
}

pub async fn rename(Path(id): Path<u32>, Json(Body { name }): Json<Body>) -> Response {
    Json(()).into_response()
}
