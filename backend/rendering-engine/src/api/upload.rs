use crate::api::common::*;
use crate::structs::{ImageState, UploadAssetRequest};
use axum_typed_multipart::TypedMultipart;
use std::path::Path;

// TODO: Move functions to io.rs and split into smaller functions.
pub async fn upload(
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
    if crate::db::contains(&image_name_no_ext, &state.pool).await {
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
    let Ok(directory_path) = crate::io::create(&image_name_no_ext).await else {
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
    let Ok(metadata) = crate::io::convert(&image_path, &store_path).await else {
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
    let _ = crate::db::insert(
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