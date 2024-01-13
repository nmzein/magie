mod db;
mod decoders;
mod io;
mod structs;
mod traits;

use crate::structs::{AppState, ImageState, Plugin, Selection, UploadAssetRequest};

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

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
use anyhow::Result;

use tower_http::cors::CorsLayer;
use libloading::{Library, Symbol};

// TODO: Remove.
use openslide_rs::OpenSlide;

static PLUGIN_PATH: &str = "src/plugins";

#[tokio::main]
async fn main() {
    let pool = db::connect().await.unwrap();

    let state = AppState {
        pool,
        plugins: load_plugins().await.unwrap_or(HashMap::new()),
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
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// TODO: Load plugins based on feature flags.
async fn load_plugins() -> Result<HashMap<String, Plugin>> {
    let mut plugins = HashMap::new();

    // Iterate over the files in the plugin directory
    for entry in std::fs::read_dir(PLUGIN_PATH)? {
        let entry = entry?;
        let path = entry.path();

        // Load each dynamic library as a plugin
        if path.is_file() && path.extension().map_or(false, |ext| ext == "so" || ext == "dylib" || ext == "dll") {
            // Load the `name` function from the plugin
            unsafe {
                let library = Library::new(&path)?;
                let name: Symbol<fn() -> String> = library.get(b"name")?;

                plugins.insert(name(), Plugin { name: *name });
            }
        }
    }

    log::<String>(
        StatusCode::OK,
        &format!("Loaded plugin(s): {:?}.", plugins.keys().cloned().collect::<Vec<_>>()),
        None,
    ).await;

    Ok(plugins)
}

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

async fn annotation_generators(Extension(state): Extension<AppState>) -> Response {
    log::<String>(
        StatusCode::ACCEPTED,
        "Received request for annotation generators.",
        None,
    )
    .await;

    Json(state.plugins.keys().cloned().collect::<Vec<_>>()).into_response()
}

async fn connect(ws: WebSocketUpgrade, Extension(state): Extension<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        render(socket, state).await;
    })
}

// TODO: Send error messages to frontend.
async fn render(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(Message::Text(message))) = socket.recv().await {
        if let Ok(selection) = serde_json::from_str::<Selection>(&message) {
            log::<String>(
                StatusCode::ACCEPTED,
                &format!("Received selection: {:?}.", selection),
                None,
            )
            .await;

            if let Ok(Some(image)) = db::get(&selection.image_name, &state.pool).await {
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

    match db::get(&image_name, &state.pool).await {
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

async fn annotations(Extension(state): Extension<AppState>, image_name: String) -> Response {
    if !db::contains(&image_name, &state.pool).await {
        return log_respond::<String>(
            StatusCode::BAD_REQUEST,
            &format!("Image with name {} does not exist.", image_name),
            None,
        )
        .await;
    }

    let Ok(annotations) = io::annotations(&image_name).await else {
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

    // Log successful saving of image to disk.
    log::<String>(
        StatusCode::CREATED,
        "Successfully saved image to disk.",
        None,
    )
    .await;

    // Convert image to ZARR.
    let store_path = PathBuf::from(format!(
        "store/{}/{}.zarr",
        image_name_no_ext, image_name_no_ext
    ));

    // TODO: Check file extension within function and choose decoder based on this.
    let Ok(metadata) = io::convert::<OpenSlide>(&image_path, &store_path).await else {
        return log_respond::<String>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to convert the image to zarr.",
            None,
        )
        .await;
    };

    // Insert image into database.
    let _ = db::insert(
        image_name_no_ext,
        &ImageState {
            image_path,
            store_path,
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

    log::<String>(
        StatusCode::CREATED,
        "Successfully saved image metadata to database.",
        None,
    )
    .await;

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
        let annotations_file_path = directory_path.join(&annotations_file_name);
        let _ = annotations.contents.persist(&annotations_file_path).map_err(|e| async {
            return log_respond(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save annotations with name {} to disk.", annotations_file_name),
                Some(e),
            )
            .await;
        });

        // TODO: Check that file is in correct format given annotation generator.
        // Log successful saving of annotations to disk.
        log_respond::<String>(
            StatusCode::CREATED,
            "Successfully saved annotations to disk.",
            None,
        )
        .await
    } else {
        // TODO: Generate annotations.
        log_respond::<String>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        )
        .await
    }
}

async fn delete(Extension(state): Extension<AppState>, image_name: String) -> Response {
    // Remove from fs.
    let _ = io::remove(&image_name).await.map_err(|e| async {
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

    // Remove entry from db.
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
