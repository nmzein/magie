mod json;
mod image;

use std::env;
use serde::{Deserialize, Serialize};
use axum::{
    // extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    // extract::connect_info::ConnectInfo,
    response::{Json, IntoResponse, Response},
    routing::get,
    Router, Server
};
// use futures::{sink::SinkExt, stream::StreamExt};

// TODO: Move to data file
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    predictions: Vec<u64>,
    coordinates: Vec<Vec<u64>>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/process-image", get(process_image));
        // .route("/api/load-image", ws(load_image));

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn process_image() -> Response {
    // TODO: Receive image either from client or query third party server - for now we provide our own image.
    let image_name = "image.tiff";
    let image_path = PathBuf::from(format!("images/{}", image_name));

    // TODO: Parallelise this.
    // TODO: Compute annotation data - for now we provide premade data.
    // let json_path = annotation::compute(image_path);
    let json_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("plugins/tiatoolbox/patch_prediction/0.raw.json");

    // TODO: Parse annotation data in JSON form and store in sqlite3 database.
    let json_data = json::parse::<Data>(&json_path);

    // Process image into HDF5 file.
    let image_name_without_extension = image_path.file_stem();
    let hdf5_path = PathBuf::from(format!("{}.h5", image_name_without_extension));
    
    image::process(&image_path, &hdf5_path);

    // TODO: Store HDF5 file path in state for future uses.

    // TODO: Query HDF5 and return preliminary image.
    Json(()).into_response()
}


// async fn load_image(ws: ws::WsUpgrade) {
//     // Upgrade the HTTP connection to a WebSocket.
//     let ws = ws
//         .web_socket()
//         .await
//         .expect("Failed to upgrade to WebSocket");


//     while let Ok(bytes) = file.read(&mut [0u8; 1024]) {
//         let frame = ws::Frame::binary(bytes);
//         ws.send(frame).await.expect("Failed to send WebSocket frame");
//     }
// }
