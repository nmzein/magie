pub use crate::structs::AppState;
pub use axum::{Extension, http::StatusCode, response::{IntoResponse, Json, Response}};

use std::fmt::Debug;

pub async fn log_respond<T: Debug>(
    status_code: StatusCode,
    message: &str,
    details: Option<T>,
) -> Response {
    log::<T>(status_code, message, details).await;

    (status_code, String::from(message)).into_response()
}

pub async fn log<T: Debug>(status_code: StatusCode, message: &str, details: Option<T>) {
    if status_code.is_success() {
        println!("Ok <{}>: {}", status_code, message);
        if let Some(details) = details {
            println!("Details: {:?}", details);
        }
    } else {
        eprintln!("Error <{}>: {}", status_code, message);
        if let Some(details) = details {
            eprintln!("Details: {:?}", details);
        }
    }

    println!();
}
