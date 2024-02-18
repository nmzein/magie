use crate::api::common::*;
use crate::structs::{ImageState, UploadAssetRequest};
use axum_typed_multipart::TypedMultipart;
use std::path::Path;

// TODO: Move functions to io.rs and split into smaller functions.
pub async fn upload(
    Extension(AppState { pool, .. }): Extension<AppState>,
    TypedMultipart(UploadAssetRequest {
        image,
        annotations,
        annotation_generator,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    // Get image name from metadata request body.
    let Some((image_name, image_name_no_ext)) = image
        .metadata
        .file_name
        .as_ref()
        .and_then(|name| Some((name, Path::new(name).file_stem()?.to_str()?)))
    else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve image name or convert to string.",
            None,
        );
    };

    // Log successful parsing of image file name.
    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request to process image with name: {}.",
            image_name
        ),
        None,
    );

    // Check if image already exists in database.
    if crate::db::contains(&image_name_no_ext, &pool).await {
        return log_respond::<()>(
            StatusCode::BAD_REQUEST,
            &format!(
                "Image with name {} already exists. Consider deleting it from the list first.",
                image_name_no_ext
            ),
            None,
        );
    }

    // Create a directory in store for the image.
    let Ok(directory_path) = crate::io::create(&image_name_no_ext).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!(
                "Failed to create directory for image with name {}.",
                image_name_no_ext
            ),
            None,
        );
    };

    // Save image to disk.
    let image_path = directory_path.join(&image_name);
    let _ = image.contents.persist(&image_path).map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to save image with name {} to disk.", image_name),
            Some(e),
        );
    });

    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::CREATED,
        "Successfully saved image to disk.",
        None,
    );

    // TODO: Check file extension within function and choose decoder based on this.
    // Convert image to ZARR.
    let store_path = directory_path.join(&format!("{}.zarr", image_name_no_ext));
    let Ok(metadata) = crate::io::convert(&image_path, &store_path).await else {
        return log_respond::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to convert the image to zarr.",
            None,
        );
    };

    #[cfg(feature = "log")]
    log::<()>(
        StatusCode::CREATED,
        "Successfully converted image to zarr.",
        None,
    );

    let mut annotations_path = None;
    if let Some(annotations) = annotations {
        // Get annotations file name from metadata request body.
        let Some(annotations_file_name) = annotations.metadata.file_name else {
            return log_respond::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve annotations file name from metadata request body.",
                None,
            );
        };

        // TODO: Check that file is in correct format given annotation generator.

        // Save annotations to disk.
        let local_annotations_path = directory_path.join(&annotations_file_name);
        let _ = annotations
            .contents
            .persist(&local_annotations_path)
            .map_err(|e| async {
                return log_respond(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!(
                        "Failed to save annotations with name {} to disk.",
                        annotations_file_name
                    ),
                    Some(e),
                );
            });
        annotations_path = Some(local_annotations_path);

        // Log successful saving of annotations to disk.
        #[cfg(feature = "log")]
        log::<()>(
            StatusCode::CREATED,
            "Successfully saved annotations to disk.",
            None,
        );
    } else {
        // TODO: Generate annotations.
        #[cfg(feature = "log")]
        log::<()>(
            StatusCode::CREATED,
            "No annotations provided. TODO: Generate annotations.",
            None,
        );
    }

    // Insert into database.
    let _ = crate::db::insert(
        &image_name_no_ext,
        &ImageState {
            image_path,
            store_path,
            annotations_path,
            metadata,
        },
        &pool,
    )
    .await
    .map_err(|e| async {
        return log_respond(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save image metadata to database.",
            Some(e),
        );
    });

    log_respond::<()>(
        StatusCode::CREATED,
        "Successfully saved image metadata to database.",
        None,
    )
}
