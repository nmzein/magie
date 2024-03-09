use crate::api::common::*;
use crate::structs::UploadAssetRequest;
use axum_typed_multipart::TypedMultipart;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

// TODO: Split into smaller functions.
pub async fn upload(
    Extension(AppState { conn, decoders, .. }): Extension<AppState>,
    TypedMultipart(UploadAssetRequest {
        directory_path,
        image,
        annotations,
        annotation_generator,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    // Get image name from metadata request body.
    let (image_name, image_name_no_ext) = match image
        .metadata
        .file_name
        .as_ref()
        .and_then(|name| Some((name, Path::new(name).file_stem()?.to_str()?)))
    {
        Some((name, name_no_ext)) => {
            #[cfg(feature = "log-success")]
            log::<()>(
                StatusCode::ACCEPTED,
                &format!("Received request to process image with name: {}.", name),
                None,
            );

            (name, name_no_ext)
        }
        None => {
            let resp = log::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve image name or convert to string.",
                None,
            );

            return resp;
        }
    };

    // Check if image already exists in database.
    let directory_path = PathBuf::from(directory_path).join(image_name_no_ext);
    if crate::db::contains(&directory_path.to_str().unwrap(), Arc::clone(&conn)).await {
        let resp = log::<()>(
            StatusCode::BAD_REQUEST,
            &format!(
                "Image with name {} already exists. Consider deleting it from the list first.",
                image_name_no_ext
            ),
            None,
        );

        return resp;
    }

    // Create a directory in store for the image.
    let _ = crate::io::create(&directory_path).await.map_err(|e| async {
        let resp = log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Failed to create directory for image with name {}.",
                image_name_no_ext
            ),
            Some(e),
        );

        return resp;
    });

    // Save image to disk.
    let image_path = directory_path.join(&image_name);
    match crate::io::save_asset(image.contents, &image_path).await {
        Ok(_) => {
            #[cfg(feature = "log-success")]
            log::<()>(
                StatusCode::CREATED,
                "Successfully saved image to disk.",
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save image with name {} to disk.", image_name),
                Some(e),
            );

            return resp;
        }
    }

    // Convert image to ZARR.
    let store_name = format!("{image_name_no_ext}.zarr");
    let store_path = directory_path.join(&store_name);
    let metadata = match crate::io::convert(&image_path, &store_path, Arc::clone(&decoders)).await {
        Ok(metadata) => {
            if metadata.is_empty() {
                let resp = log::<()>(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to convert image to ZARR. No metadata returned.",
                    None,
                );

                return resp;
            }

            #[cfg(feature = "log-success")]
            log::<()>(
                StatusCode::CREATED,
                "Successfully converted image to ZARR.",
                None,
            );

            metadata
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to convert the image to ZARR.",
                Some(e),
            );

            return resp;
        }
    };

    let mut annotations_name = None;
    if let Some(annotations) = annotations {
        // Get annotations file name from metadata request body.
        let Some(local_annotations_name) = annotations.metadata.file_name else {
            let resp = log::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve annotations file name from metadata request body.",
                None,
            );

            return resp;
        };

        // TODO: Check that file is in correct format given annotation generator.

        // Save annotations to disk.
        let annotations_path = directory_path.join(&local_annotations_name);
        match crate::io::save_asset(annotations.contents, &annotations_path).await {
            Ok(_) => {
                #[cfg(feature = "log-success")]
                log::<()>(
                    StatusCode::CREATED,
                    "Successfully saved annotations to disk.",
                    None,
                );

                annotations_name = Some(local_annotations_name);
            }
            Err(e) => {
                let resp = log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!(
                        "Failed to save annotations with name {:?} to disk.",
                        annotations_name
                    ),
                    Some(e),
                );

                return resp;
            }
        }
    } else {
        // TODO: Generate annotations.
        log::<()>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        );
    }

    // Insert into database.
    match crate::db::insert(
        &directory_path,
        &image_name,
        &store_name,
        annotations_name.as_deref(),
        metadata,
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
