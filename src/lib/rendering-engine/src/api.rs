mod json;
mod image;

use std::env;
use serde::{Deserialize, Serialize};
use axum::{Router, Server};
use axum::routing::get;
use axum::response::{Json, IntoResponse, Response};

// TODO: Move to data file
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    predictions: Vec<u64>,
    coordinates: Vec<Vec<u64>>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/api/data", get(get_data));

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_data() -> Response {
    let json_file_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("plugins/tiatoolbox/patch_prediction/0.raw.json");
    let tiff_file_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("images/image.tiff");
    let svs_file_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("images/image.svs");

    let data0 = match json::parse::<Data>(json_file_path) {
        Ok(d) => d,
        Err(_e) => panic!("Error"),
    };
  
    let data1 = match image::decode(tiff_file_path) {
        Ok(d) => d,
        Err(_e) => panic!("Error"),
    };

    let data2 = match image::decode(svs_file_path) {
        Ok(d) => d,
        Err(_e) => panic!("Error"),
    };

    Json((data1, data2, data0)).into_response()
}