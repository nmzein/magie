mod db;
mod decoders;
mod io;
mod structs;
mod traits;
mod generators;

use crate::structs::{AppState, ImageState, Selection, UploadAssetRequest};
// use crate::structs::Generator;
// use crate::traits::AnnotationGenerator;

use std::fmt::Display;
use std::path::Path;
use std::collections::HashMap;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        DefaultBodyLimit, Extension, WebSocketUpgrade,
    },
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use axum_typed_multipart::TypedMultipart;
use tower_http::cors::CorsLayer;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
// use libloading::{Library, Symbol};

// static GENERATORS_PATH: &str = "src/generators";
// static STORE_PATH: &str = "../store";

#[tokio::main]
async fn main() {
    let pool = db::connect().await.unwrap();

    // let mut generators: HashMap<String, Generator> = HashMap::new();
    
    // generators.insert(crate::generators::tiatoolbox::name(), Generator {
    //     name: crate::generators::tiatoolbox::name(),
    //     read_annotations: read_annotations,
    // });

    // let generators = load_generators().await.unwrap_or(HashMap::new());

    let state = AppState {
        pool,
        // generators,
        generators: HashMap::new(),
    };

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
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// TODO: Load generators based on feature flags.
// async fn load_generators() -> Result<HashMap<String, Generator>> {
//     let mut generators = HashMap::new();

//     // Iterate over the files in the Generator directory
//     for entry in std::fs::read_dir(GENERATORS_PATH)? {
//         let entry = entry?;
//         let path = entry.path();

//         // Load each dynamic library as a Generator
//         if path.is_file() && path.extension().map_or(false, |ext| ext == "so" || ext == "dylib" || ext == "dll") {
//             // Load the `name` and `read_annotations` function from the Generator.
//             unsafe {
//                 let library = Library::new(&path)?;
//                 let name: Symbol<fn() -> String> = library.get(b"name")?;
//                 let read_annotations: Symbol<fn(image_path: &str) -> Vec<AnnotationLayer>> = library.get(b"read_annotations")?;

//                 generators.insert(name(), Generator { name: name(), read_annotations: *read_annotations});
//             }
//         }
//     }

//     log::<String>(
//         StatusCode::OK,
//         &format!("Loaded Generator(s): {:?}.", generators.keys().cloned().collect::<Vec<_>>()),
//         None,
//     ).await;

//     Ok(generators)
// }

async fn image_list(Extension(state): Extension<AppState>) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for list of images.",
        None,
    )
    .await;

    if let Ok(images) = db::list(&state.pool).await {
        return Json(images).into_response();
    }

    log_respond::<String>(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to retrieve list of images.",
        None,
    )
    .await
}

async fn annotation_generators(Extension(_state): Extension<AppState>) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    )
    .await;

    // Json(state.generators.keys().cloned().collect::<Vec<_>>()).into_response()
    Json(["TIA Toolbox".to_string()]).into_response()
}

async fn connect(ws: WebSocketUpgrade, Extension(state): Extension<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        render(socket, state).await;
    })
}

// TODO: Send error messages to frontend.
async fn render(socket: WebSocket, state: AppState) {
    let (mut sink, mut stream) = socket.split();
    // Credit: https://gist.github.com/hexcowboy/8ebcf13a5d3b681aa6c684ad51dd6e0c
    // Create an mpsc so we can send messages to the sink from multiple threads.
    let (sender, mut receiver) = mpsc::channel::<Message>(4);

    // Spawn a task that forwards messages from the mpsc receiver to the websocket sink.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if sink.send(message.into()).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(Message::Text(message))) = stream.next().await {
        if let Ok(selection) = serde_json::from_str::<Selection>(&message) {
            log::<String>(
                StatusCode::ACCEPTED,
                &format!("Received selection: {:?}.", selection),
                None,
            )
            .await;

            if let Ok((_, store_path, _)) = db::get_paths(&selection.image_name, &state.pool).await {
                let _ = io::retrieve(&store_path.into(), selection.clone(), sender.clone())
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
                &format!("Failed to parse selection: {}.", message),
                None,
            )
            .await;
        }
    }
}

async fn metadata(Extension(state): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for metadata of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    match db::get_metadata(&image_name, &state.pool).await {
        Ok(metadata) => Json(metadata).into_response(),
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

async fn annotations(Extension(state): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    let Ok((_, _, Some(annotations_path))) = db::get_paths(&image_name, &state.pool).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Image with name {} does not exist in the database or doesn't have annotations.",
                image_name
            ),
            None,
        )
        .await;
    };

    let Ok(annotations) = io::annotations(&annotations_path).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve annotations.",
            None,
        )
        .await;
    };
    
    log::<String>(StatusCode::OK, "Successfully retrieved annotations.", None).await;

    Json(annotations).into_response()
}

// TODO: Move functions to io.rs and split into smaller functions.
async fn upload(
    Extension(state): Extension<AppState>,
    TypedMultipart(UploadAssetRequest {
        image,
        annotations,
        annotation_generator,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    // Get image name from metadata request body.
    let Some(image_name) = image.metadata.file_name else {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            "Failed to retrieve image name from metadata request body.",
            None,
        ).await;
    };

    // Strip file extension.
    let Some(image_name_no_ext) = Path::new(&image_name).file_stem() else {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            "Failed to remove image file extension.",
            None,
        ).await;
    };

    // Convert OsStr to &str.
    let Some(image_name_no_ext) = image_name_no_ext.to_str() else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to convert image name to string.",
            None,
        ).await;
    };
    
    // Log successful parsing of image file name.
    log::<String>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request to process image with name: {}.",
            image_name
        ),
        None,
    )
    .await;

    // Check if image already exists in database.
    if db::contains(&image_name_no_ext, &state.pool).await {
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

    // Create a directory in store for the image.
    let Ok(directory_path) = io::create(&image_name_no_ext).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Failed to create directory for image with name {}.",
                image_name_no_ext
            ),
            None,
        )
        .await;
    };

    // Save image to disk.
    let image_path = directory_path.join(&image_name);
    let _ = image.contents.persist(&image_path).map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to save image with name {} to disk.", image_name),
            Some(e),
        )
        .await;
    });

    log::<String>(
        StatusCode::CREATED,
        "Successfully saved image to disk.",
        None,
    )
    .await;

    // TODO: Check file extension within function and choose decoder based on this.
    // Convert image to ZARR.
    let store_path = directory_path.join(&format!("{}.zarr", image_name_no_ext));
    let Ok(metadata) = io::convert(&image_path, &store_path).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to convert the image to zarr.",
            None,
        )
        .await;
    };

    log::<String>(
        StatusCode::CREATED,
        "Successfully converted image to zarr.",
        None,
    ).await;

    let mut annotations_path = directory_path;
    if let Some(annotations) = annotations {
        // Get annotations file name from metadata request body.
        let Some(annotations_file_name) = annotations.metadata.file_name else {
            return log_respond::<String>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve annotations file name from metadata request body.",
                None,
            ).await;
        };
    
        // Save annotations to disk.
        annotations_path = annotations_path.join(&annotations_file_name);
        let _ = annotations.contents.persist(&annotations_path).map_err(|e| async {
            return log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save annotations with name {} to disk.", annotations_file_name),
                Some(e),
            )
            .await;
        });

        // TODO: Check that file is in correct format given annotation generator.
        // Log successful saving of annotations to disk.
        log::<String>(
            StatusCode::CREATED,
            "Successfully saved annotations to disk.",
            None,
        )
        .await;
    } else {
        // TODO: Generate annotations.
        log::<String>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        )
        .await;
    }

    // Insert into database.
    let _ = db::insert(
        image_name_no_ext,
        &ImageState {
            image_path,
            store_path,
            annotations_path: Some(annotations_path),
            metadata,
        },
        &state.pool,
    ).await.map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save image metadata to database.",
            Some(e),
        )
        .await;
    });

    log_respond::<String>(
        StatusCode::CREATED,
        "Successfully saved image metadata to database.",
        None,
    )
    .await
}

async fn delete(Extension(state): Extension<AppState>, image_name: String) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        &format!("Received request to delete image with name: {}.", image_name),
        None,
    )
    .await;

    // Delete directory from fs.
    let _ = io::delete(&image_name).await.map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Could not delete directory for image with name {}.",
                image_name
            ),
            Some(e),
        )
        .await;
    });

    // Remove entries from db.
    let _ = db::remove(&image_name, &state.pool).await.map_err(|e| async {
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
