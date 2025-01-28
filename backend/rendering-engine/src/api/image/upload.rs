use crate::api::common::*;
use crate::types::UploadAssetRequest;
use axum_typed_multipart::{FieldData, TypedMultipart};
use shared::structs::{AnnotationLayer, MetadataLayer};
use std::{path::PathBuf, process::Command};
use tempfile::NamedTempFile;

#[derive(Deserialize)]
pub struct Params {
    pub parent_id: u32,
    pub name: String,
}

const UPLOADED_IMAGE_NAME: &str = "uploaded_image";
const UPLOADED_ANNOTATIONS_NAME: &str = "uploaded_annotations";
const ENCODED_IMAGE_NAME: &str = "encoded_image.zarr";
const TRANSLATED_ANNOTATIONS_NAME: &str = "translated_annotations.json";

// TODO: Perform checks on files before saving them to avoid malware.
pub async fn upload(
    Extension(conn): Extension<AppState>,
    Path(Params { parent_id, name }): Path<Params>,
    TypedMultipart(UploadAssetRequest {
        decoder,
        encoder,
        generator,
        image_file,
        annotations_file,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    let name = &name;

    #[cfg(feature = "log.request")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!("Received upload assets."),
        None,
    );

    if !encoders::export::names().contains(&encoder.as_str()) {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            &format!("Encoder with name `{encoder}` could not be found."),
            None,
        );
    }

    if let Some(generator) = &generator {
        if !generators::export::names().contains(&generator.as_str()) {
            return log::<()>(
                StatusCode::BAD_REQUEST,
                &format!("Generator with name `{generator}` could not be found."),
                None,
            );
        }
    }

    let image_metadata = image_file.metadata.clone();
    // Extract image extension from metadata request body.
    let upl_img_ext = match image_metadata.file_name.as_ref().map(std::path::Path::new) {
        Some(name) => match name.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => {
                return log::<()>(
                    StatusCode::BAD_REQUEST,
                    "Failed to retrieve image extension from file metadata.",
                    None,
                );
            }
        },
        None => {
            return log::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve image extension from file metadata.",
                None,
            );
        }
    };

    // Check if image already exists in database.
    match crate::db::image::exists(parent_id, &name, Arc::clone(&conn)) {
        Ok(true) => {
            return log::<()>(
                StatusCode::BAD_REQUEST,
                &format!(
                    "Image with name `{name}` already exists in directory with id `{parent_id}`. Consider deleting it from the list first."
                ),
                None,
            );
        }
        Ok(false) => { /* Image does not exist in database, continue. */ }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!(
                    "State database failed to check if image with name `{name}` already exists in directory with id `{parent_id}`.",
                ),
                Some(e),
            );
        }
    }

    // The image's directory path consists of the concatenation of
    // its parent directory's path and its name without extension.
    let path = match crate::db::directory::path(parent_id, Arc::clone(&conn)) {
        Ok(path) => path.join(name),
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to retrieve directory path for image with name `{name}`.",),
                Some(e),
            );
        }
    };

    // Create a directory in local store for the image.
    let _ = crate::io::create(&path).await.map_err(|e| {
        return log(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to create a directory for image with name `{name}`."),
            Some(e),
        );
    });

    let metadata_layers = match handle_image(image_file, &path, &name, &upl_img_ext, &encoder).await
    {
        Ok(layers) => layers,
        Err(resp) => return resp,
    };

    let mut annotations_ext = None;
    let mut annotation_layers = Vec::new();
    if let Some(generator) = generator {
        (annotations_ext, annotation_layers) =
            match handle_annotations(&path, annotations_file, generator).await {
                Ok((name, layers)) => (Some(name), layers),
                Err(resp) => return resp,
            };
    }

    // Insert into database.
    match crate::db::image::insert(
        parent_id,
        &name,
        &upl_img_ext,
        &upl_img_ext,
        &encoder,
        annotations_ext.as_deref(),
        metadata_layers,
        annotation_layers,
        Arc::clone(&conn),
    ) {
        Ok(_) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                "Successfully saved uploaded file(s) metadata to database.",
                None,
            );
        }
        Err(e) => {
            return log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save uploaded file(s) metadata to database.",
                Some(e),
            );
        }
    };

    match crate::db::general::get_registry(Arc::clone(&conn)) {
        Ok(registry) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::OK,
                "Successfully retrieved registry from the state database.",
                None,
            );

            Json(registry).into_response()
        }
        Err(e) => log(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve registry from the state database.",
            Some(e),
        ),
    }
}

async fn handle_image(
    file: FieldData<NamedTempFile>,
    path: &PathBuf,
    name: &str,
    upl_img_ext: &str,
    encoder: &str,
) -> Result<Vec<MetadataLayer>, Response> {
    // Path where the uploaded image will be stored.
    let upl_img_path = path.join(&format!("{UPLOADED_IMAGE_NAME}.{upl_img_ext}"));

    // Path where the encoded image will be stored.
    let enc_img_path = path.join(ENCODED_IMAGE_NAME);

    let thumbnail_path = path.join("thumbnail.jpg");

    // Save image to disk.
    match crate::io::save_asset(file.contents, &upl_img_path).await {
        Ok(_) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully saved image with name `{name}` to disk."),
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save image with name `{name}` to disk."),
                Some(e),
            );

            return Err(resp);
        }
    }

    // Encode image to Zarr derivative format.
    match crate::io::convert(&upl_img_path, &enc_img_path, &thumbnail_path, encoder).await {
        Ok(metadata) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully converted image with name `{name}` to Zarr."),
                None,
            );

            return Ok(metadata);
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to convert the image with name `{name}` to Zarr."),
                Some(e),
            );

            return Err(resp);
        }
    };
}

async fn handle_annotations(
    path: &PathBuf,
    file: Option<FieldData<NamedTempFile>>,
    generator_name: String,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // Get annotations generator.
    let Some(generator) = generators::export::get(&generator_name) else {
        let resp = log::<()>(
            StatusCode::NOT_FOUND,
            &format!("Generator with name `{generator_name}` could not be found."),
            None,
        );

        return Err(resp);
    };

    // If user provides annotations file, translate it and compute buffer geometries.
    // Else generate annotations and compute buffer geometries.
    match file {
        None => generate_annotations(path, generator).await,
        Some(file) => translate_annotations(path, file, generator).await,
    }
}

async fn generate_annotations(
    _path: &PathBuf,
    _generator: Box<dyn Generator>,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // TODO: Generate annotations.
    log::<()>(
        StatusCode::CREATED,
        "No annotations provided. TODO: Generate annotations.",
        None,
    );

    return Ok((String::from("TODO"), Vec::new()));
}

async fn translate_annotations(
    path: &PathBuf,
    file: FieldData<NamedTempFile>,
    generator: Box<dyn Generator>,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // Get uploaded annotation file's extension from metadata request body.
    // TODO: Fix unwrap.
    let upl_anno_ext = match std::path::Path::new(file.metadata.file_name.as_ref().unwrap())
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some(ext) => ext,
        None => {
            let resp = log::<()>(
                StatusCode::BAD_REQUEST,
                "Failed to retrieve annotation file extension from file metadata.",
                None,
            );

            return Err(resp);
        }
    };

    // Path where the uploaded annotations file will be stored.
    let upl_anno_path: PathBuf = path.join(&format!("{UPLOADED_ANNOTATIONS_NAME}.{upl_anno_ext}"));

    // Save uploaded annotations file to disk.
    match crate::io::save_asset(file.contents, &upl_anno_path).await {
        Ok(_) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully saved annotations file to disk."),
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to save annotations file to disk."),
                Some(e),
            );

            return Err(resp);
        }
    }

    // Translate annotations.
    let annotation_layers = match generator.translate(&upl_anno_path) {
        Ok(layers) => layers,
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to translate annotations file with name using the generator."),
                Some(e),
            );

            return Err(resp);
        }
    };

    // TODO: Use capnproto rather than storing an intermediate translated annotations json file.
    // TODO: Or, try writing own buffer geometry creator/gltf lib in Rust.
    // Serialize annotation layers.
    let Ok(serialized_annotation_layers) = serde_json::to_string(&annotation_layers) else {
        let resp = log::<()>(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize annotation layers.",
            None,
        );

        return Err(resp);
    };

    // Save json string to file.
    let transl_anno_path = path.join(TRANSLATED_ANNOTATIONS_NAME);
    std::fs::write(transl_anno_path.clone(), serialized_annotation_layers)
        .expect("Unable to write file");

    // Compute annotation positions and normals.
    match Command::new("node")
        .arg("--max-old-space-size=4096")
        .arg("./geometry-computer/index.js")
        .arg(transl_anno_path)
        .arg(path)
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

    return Ok((upl_anno_ext.to_owned(), annotation_layers));
}
