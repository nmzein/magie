use crate::api::common::*;
use crate::consts::LOCAL_STORE_PATH;
use crate::types::{MetadataLayer, UploadAssetRequest};
use axum_typed_multipart::{FieldData, TypedMultipart};
use shared::structs::AnnotationLayer;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};
use tempfile::NamedTempFile;

pub async fn upload(
    Extension(conn): Extension<AppState>,
    TypedMultipart(UploadAssetRequest {
        parent_directory_path,
        image_file,
        annotations_file,
        generator_name,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    let image_metadata = image_file.metadata.clone();
    // Get image name from metadata request body.
    let (image_name, image_name_no_ext) = match image_metadata
        .file_name
        .as_ref()
        .and_then(|name| Some((name, Path::new(name).file_stem()?.to_str()?)))
    {
        Some((name, name_no_ext)) => {
            #[cfg(feature = "log.request")]
            log::<()>(
                StatusCode::ACCEPTED,
                &format!("Received request to process upload for image with name: {name}."),
                None,
            );

            (name, name_no_ext)
        }
        None => {
            return log::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve image name from file metadata or failed to convert it to a string.",
                None,
            );
        }
    };
    let directory_path = PathBuf::from(parent_directory_path).join(image_name_no_ext);

    // Check if image already exists in database.
    match crate::db::contains(&directory_path.to_str().unwrap(), Arc::clone(&conn)).await {
        Ok(true) => {
            return log::<()>(
                StatusCode::BAD_REQUEST,
                &format!(
                    "Image with name `{image_name_no_ext}` already exists. Consider deleting it from the list first."
                ),
                None,
            );
        }
        Ok(false) => { /* Image does not exist in database, continue. */ }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "State database failed to check if image with name `{image_name_no_ext}` already exists.",
                ),
                Some(e),
            );
        }
    }

    // Create a directory in backend/store.
    let _ = crate::io::create(&directory_path).await.map_err(|e| async {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to create a directory for image with name `{image_name_no_ext}`."),
            Some(e),
        );
    });

    // Path to uploaded image file.
    let image_path = directory_path.join(&image_name);

    // Path to Zarr store.
    let store_name = format!("{image_name_no_ext}.zarr");
    let store_path = directory_path.join(&store_name);

    let metadata_layers =
        match handle_image(image_file, &image_path, &image_name, &store_path).await {
            Ok(layers) => layers,
            Err(resp) => return resp,
        };

    let (annotations_name, annotation_layers) =
        match handle_annotations(&directory_path, annotations_file, generator_name).await {
            Ok((name, layers)) => (Some(name), layers),
            Err(resp) => return resp,
        };

    // Insert into database.
    match crate::db::insert(
        &directory_path,
        &image_name,
        &store_name,
        annotations_name.as_deref(),
        metadata_layers,
        annotation_layers,
        Arc::clone(&conn),
    )
    .await
    {
        Ok(_) => log::<()>(
            StatusCode::CREATED,
            "Successfully saved image to database.",
            None,
        ),
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save image to database.",
            Some(e),
        ),
    }
}

async fn handle_image(
    image_file: FieldData<NamedTempFile>,
    image_path: &PathBuf,
    image_name: &str,
    store_path: &PathBuf,
) -> Result<Vec<MetadataLayer>, Response> {
    // Save image to disk.
    match crate::io::save_asset(image_file.contents, &image_path).await {
        Ok(_) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully saved image with name `{image_name}` to disk."),
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save image with name `{image_name}` to disk."),
                Some(e),
            );

            return Err(resp);
        }
    }

    // Convert image to Zarr.
    match crate::io::convert(&image_path, &store_path).await {
        Ok(metadata) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully converted image with name `{image_name}` to Zarr."),
                None,
            );

            return Ok(metadata);
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to convert the image with name `{image_name}` to Zarr."),
                Some(e),
            );

            return Err(resp);
        }
    };
}

async fn handle_annotations(
    directory_path: &PathBuf,
    annotations_file: Option<FieldData<NamedTempFile>>,
    generator_name: String,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // If no annotations file provided by the user, generate them.
    let Some(annotations_file) = annotations_file else {
        // TODO: Generate annotations.
        log::<()>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        );

        return Ok((String::from("TODO"), Vec::new()));
    };

    // Get annotations file name from metadata request body.
    let Some(annotations_name) = annotations_file.metadata.file_name else {
        let resp = log::<()>(
            StatusCode::BAD_REQUEST,
            "Failed to retrieve filename from annotations file metadata.",
            None,
        );

        return Err(resp);
    };

    // Save uploaded annotations file to disk.
    let annotations_path = directory_path.join(&annotations_name);
    match crate::io::save_asset(annotations_file.contents, &annotations_path).await {
        Ok(_) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!(
                    "Successfully saved annotations file with name `{annotations_name}` to disk."
                ),
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save annotations file with name `{annotations_name}` to disk."),
                Some(e),
            );

            return Err(resp);
        }
    }

    // Get annotations generator.
    let Some(generator) = generators::export::get(&generator_name) else {
        let resp = log::<()>(
            StatusCode::NOT_FOUND,
            &format!("Generator with name `{generator_name}` could not be found."),
            None,
        );

        return Err(resp);
    };

    // Translate annotations.
    let Ok(annotation_layers) = generator.translate(&annotations_path) else {
        let resp = log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to translate annotations file with name `{annotations_name}` using the generator: `{generator_name}`."),
            None,
        );

        return Err(resp);
    };

    // TODO: Use capnproto
    // Serialize annotation layers.
    let Ok(annotation_layers_json) = serde_json::to_string(&annotation_layers) else {
        let resp = log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize annotation layers.",
            None,
        );

        return Err(resp);
    };

    // Save json string to file.
    let annotations_path = PathBuf::from(LOCAL_STORE_PATH)
        .join(directory_path)
        .join("annotations.json");

    std::fs::write(annotations_path, annotation_layers_json).expect("Unable to write file");

    // Compute annotation positions and normals.
    match Command::new("node")
        .arg("--max-old-space-size=4096")
        .arg("./geometry-computer/index.js")
        .arg(PathBuf::from(LOCAL_STORE_PATH).join(directory_path))
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                log::<()>(
                    StatusCode::CREATED,
                    "Successfully computed annotation positions and normals.",
                    None,
                );
            } else {
                let resp = log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to compute annotation positions and normals.",
                    Some(String::from_utf8(output.stderr)),
                );

                return Err(resp);
            }
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to run geometry computation.",
                Some(e),
            );

            return Err(resp);
        }
    }

    return Ok((annotations_name, annotation_layers));
}
