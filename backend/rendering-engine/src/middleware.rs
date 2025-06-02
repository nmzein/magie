use crate::{log::Logger, types::user::User};
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};

// TODO: Implement.
pub async fn authentication(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    req.extensions_mut().insert(User { id: 0 });
    next.run(req).await
}

pub async fn logging(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    // Extract information from the request.
    let method = req.method().clone();
    let uri = req.uri();
    let path = uri.path().to_string();
    let query = uri.query().unwrap_or_default().to_string();

    let logger = Logger::start(method, path, query);

    // Pass the request information to the next middleware/handler.
    req.extensions_mut().insert(logger);

    // Call the next middleware/handler.
    next.run(req).await
}
