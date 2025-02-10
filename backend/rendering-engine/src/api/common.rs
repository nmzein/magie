pub use crate::{
    log::{Check, Error, Logger},
    types::{DeleteMode, MoveMode},
};
pub use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    Extension,
};
pub use serde::Deserialize;
pub use shared::traits::Generator;
pub use std::fmt::Debug;

pub static PRIVILEDGED: [u32; 2] = [ROOT_ID, BIN_ID];
pub static STORES: [u32; 1] = [STORE_ID];

pub static ROOT_ID: u32 = 0;
pub static BIN_ID: u32 = 1;
pub static STORE_ID: u32 = 2;

pub fn log<T: Debug>(status_code: StatusCode, message: &str, details: Option<T>) -> Response {
    if status_code.is_success() {
        println!("Ok <{status_code}>: {message}");
        if let Some(details) = details {
            println!("Details: {details:?}");
        }
        println!();
    } else {
        eprintln!("Error <{status_code}>: {message}");
        if let Some(details) = details {
            eprintln!("Details: {details:?}");
        }
        eprintln!();
    }

    (status_code, String::from(message)).into_response()
}
