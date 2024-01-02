mod db;
mod decoders;
mod io;
mod structs;
mod traits;

use crate::structs::{AppState, ImageState, Selection, UploadAssetRequest};

use std::fmt::Display;
use std::path::PathBuf;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, WebSocketUpgrade,
    },
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use axum_typed_multipart::TypedMultipart;

use tokio::fs;
use tower_http::cors::CorsLayer;

// TODO: Remove.
use openslide_rs::OpenSlide;

#[tokio::main]
async fn main() {
    let pool = db::connect().await.unwrap();

    // TODO: Move URLs to .env file.
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/api/image-list", get(image_list))
        .route("/api/annotation-generators", get(annotation_generators))
        .route("/api/connect", get(connect))
        .route("/api/upload", post(upload))
        .route("/api/metadata", post(metadata))
        .route("/api/annotations", post(annotations))
        .route("/api/delete", post(delete))
        .layer(cors)
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn image_list(Extension(pool): Extension<AppState>) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for list of images.",
        None,
    )
    .await;

    if let Ok(images) = db::list(&pool).await {
        return Json(images).into_response();
    }

    log_respond::<String>(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to retrieve list of images.",
        None,
    )
    .await
}

// TODO: Actually collect annotation generator names somehow.
async fn annotation_generators() -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    )
    .await;

    Json(["TIA Toolbox", "Example 1", "Example 2"]).into_response()
}

async fn connect(ws: WebSocketUpgrade, Extension(pool): Extension<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        render(socket, pool).await;
    })
}

// TODO: Send error messages to frontend.
async fn render(mut socket: WebSocket, pool: AppState) {
    while let Some(Ok(Message::Text(message))) = socket.recv().await {
        if let Ok(selection) = serde_json::from_str::<Selection>(&message) {
            log::<String>(
                StatusCode::ACCEPTED,
                &format!("Received selection: {:?}.", selection),
                None,
            )
            .await;

            if let Ok(Some(image)) = db::get(&selection.image_name, &pool).await {
                let _ = io::retrieve(&image.store_path, &selection, &mut socket)
                    .await
                    .map_err(|e| async {
                        log(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            &format!(
                                "Failed to retrieve image with name: {}.",
                                &selection.image_name
                            ),
                            Some(e),
                        )
                        .await;
                    });
            } else {
                log::<String>(
                    StatusCode::BAD_REQUEST,
                    &format!(
                        "Couldn't find image with name: {} in the database.",
                        &selection.image_name
                    ),
                    None,
                )
                .await;
            }
        } else {
            log::<String>(
                StatusCode::BAD_REQUEST,
                "Incorrect JSON format. Expected type <Selection>.",
                None,
            )
            .await;
        }
    }
}

async fn metadata(Extension(pool): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for metadata of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    match db::get(&image_name, &pool).await {
        Ok(Some(image)) => Json(image.metadata).into_response(),
        Ok(None) => {
            log_respond::<String>(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "Metadata for image with name {} does not exist.",
                    image_name
                ),
                None,
            )
            .await
        }
        Err(e) => {
            log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve metadata.",
                Some(e),
            )
            .await
        }
    }
}

async fn annotations(Extension(pool): Extension<AppState>, image_name: String) -> Response {
    if !db::contains(&image_name, &pool).await {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            &format!("Image with name {} does not exist.", image_name),
            None,
        )
        .await;
    }

    if let Ok(annotations) = io::annotations(&image_name).await {
        log::<String>(StatusCode::OK, "Successfully retrieved annotations.", None).await;
        return Json(annotations).into_response();
    }

    log_respond::<String>(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to retrieve annotations.",
        None,
    )
    .await
}

// TODO: Move functions to io.rs.
async fn upload(
    Extension(pool): Extension<AppState>,
    TypedMultipart(UploadAssetRequest {
        image,
        annotations,
        annotation_generator,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    // ! Remove unwrap.
    let image_name = image.metadata.file_name.unwrap();
    let image_name_no_ext = image_name.split('.').collect::<Vec<&str>>()[0];
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request to process image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    if db::contains(&image_name_no_ext, &pool).await {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            &format!(
                "Image with name {} already exists. Consider deleting it from the list first.",
                image_name_no_ext
            ),
            None,
        )
        .await;
    }

    let directory_path = format!("store/{}/", image_name_no_ext);
    // ! Remove unwrap.
    fs::create_dir_all(directory_path.clone()).await.unwrap();

    let image_path = PathBuf::from(format!("{}{}", directory_path, image_name));
    match image.contents.persist(image_path.clone()) {
        Ok(_) => {
            log::<String>(
                StatusCode::CREATED,
                "Successfully saved image to disk.",
                None,
            )
            .await;

            // Convert to ZARR.
            let store_path = PathBuf::from(format!(
                "store/{}/{}.zarr",
                image_name_no_ext, image_name_no_ext
            ));

            // TODO: Check file extension within function and choose decoder based on this.
            match io::convert::<OpenSlide>(&image_path, &store_path).await {
                Ok(metadata) => {
                    match db::insert(
                        image_name_no_ext,
                        &ImageState {
                            image_path,
                            store_path,
                            metadata,
                        },
                        &pool,
                    )
                    .await
                    {
                        Ok(_) => {
                            log::<String>(
                                StatusCode::CREATED,
                                "Successfully saved image metadata to database.",
                                None,
                            )
                            .await
                        }
                        Err(e) => {
                            return log_respond(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to save image metadata to database.",
                                Some(e),
                            )
                            .await
                        }
                    }
                }
                Err(e) => {
                    return log_respond(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to convert the image to zarr.",
                        Some(e),
                    )
                    .await
                }
            }
        }
        Err(e) => {
            return log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save image to disk.",
                Some(e),
            )
            .await
        }
    }

    if let Some(annotations) = annotations {
        // ! Remove unwrap.
        let annotations_file_name = annotations.metadata.file_name.unwrap();

        let annotations_file_path =
            PathBuf::from(format!("{}{}", directory_path, annotations_file_name));

        match annotations.contents.persist(annotations_file_path) {
            Ok(_) => {
                return log_respond::<String>(
                    StatusCode::CREATED,
                    "Successfully saved annotations to disk.",
                    None,
                )
                .await;
            }
            Err(e) => {
                return log_respond(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to save annotations to disk.",
                    Some(e),
                )
                .await;
            }
        }
    } else {
        // TODO: Generate annotations.
        return log_respond::<String>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        )
        .await;
    }
}

async fn delete(Extension(pool): Extension<AppState>, image_name: String) -> Response {
    // Remove from fs.
    if !io::remove(&image_name).await {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete directory for image with name {}.",
                image_name
            ),
            None,
        )
        .await;
    }

    // Remove entry from db.
    let _ = db::remove(&image_name, &pool).await.map_err(|e| async {
        log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete image with name {} from state database.",
                image_name
            ),
            Some(e),
        )
        .await;
    });

    log_respond::<String>(StatusCode::OK, "Successfully deleted image entry.", None).await
}

async fn log_respond<T: Display>(
    status_code: StatusCode,
    message: &str,
    details: Option<T>,
) -> Response {
    log::<T>(status_code, message, details).await;

    (status_code, String::from(message)).into_response()
}

async fn log<T: Display>(status_code: StatusCode, message: &str, details: Option<T>) {
    if status_code.is_success() {
        println!("Ok <{}>: {}", status_code, message);
        if let Some(details) = details {
            println!("Details: {}", details);
        }
    } else {
        eprintln!("Error <{}>: {}", status_code, message);
        if let Some(details) = details {
            eprintln!("Details: {}", details);
        }
    }

    println!();
}
