pub use crate::types::AppState;
pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    Extension,
};
pub use shared::traits::Generator;
use std::fmt::Debug;
pub use std::sync::Arc;

pub fn log<T: Debug>(status_code: StatusCode, message: &str, details: Option<T>) -> Response {
    if status_code.is_success() {
        println!("Ok <{}>: {}", status_code, message);
        if let Some(details) = details {
            println!("Details: {:?}", details);
        }
        println!();
    } else {
        eprintln!("Error <{}>: {}", status_code, message);
        if let Some(details) = details {
            eprintln!("Details: {:?}", details);
        }
        eprintln!();
    }

    (status_code, String::from(message)).into_response()
}
