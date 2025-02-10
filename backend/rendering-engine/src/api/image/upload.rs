use crate::api::common::*;
use crate::types::UploadAssetRequest;
use axum_typed_multipart::{FieldData, TypedMultipart};
use shared::{
    structs::{AnnotationLayer, MetadataLayer},
    traits::Encoder,
};
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Deserialize)]
pub struct Params {
    pub parent_id: u32,
    pub name: String,
}

// TODO: Handle half failed states.
// TODO: Perform checks on files before saving them to avoid malware.
pub async fn upload(
    Extension(mut logger): Extension<Logger<'_>>,
    Path(Params { parent_id, name }): Path<Params>,
    TypedMultipart(UploadAssetRequest {
        decoder,
        encoder,
        generator,
        image_file,
        annotations_file,
    }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    let (encoder, encoder_name) = match encoders::export::get(encoder.as_str()) {
        Some(e) => {
            logger.report(Check::ResourceExistence, "Encoder found.");

            (e, encoder)
        }
        None => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "IU-E00",
                "Encoder could not be found.",
                None,
            );
        }
    };

    let generator = match generator.map(|g| generators::export::get(g.as_str())) {
        Some(Some(g)) => {
            logger.report(Check::ResourceExistence, "Generator found.");

            Some(g)
        }
        Some(None) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "IU-E01",
                "Generator could not be found.",
                None,
            );
        }
        None => None,
    };

    // Check if image already exists in database.
    match crate::db::image::exists(parent_id, &name) {
        Ok(false) => {
            /* Image does not exist in database, continue. */
            logger.report(
                Check::ResourceConflict,
                "Asset with same name does not already exist in directory.",
            );
        }
        Ok(true) => {
            return logger.error(
                StatusCode::CONFLICT,
                Error::ResourceConflict,
                "IU-E02",
                "Asset with same name already exists in directory.",
                None,
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IU-E03",
                "Failed to check if asset with same name exists in directory.",
                Some(e),
            );
        }
    }

    // The image's directory path consists of the concatenation of
    // its parent directory's path and its name without extension.
    let path = match crate::db::directory::path(parent_id) {
        Ok(path) => path.join(&name),
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::DatabaseQuery,
                "IU-E04",
                "Failed to retrieve parent directory path.",
                Some(e),
            );
        }
    };

    // Create a directory in local store for the image.
    match crate::io::create(&path) {
        Ok(()) => {}
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IU-E05",
                "Failed to create a directory for asset.",
                Some(e),
            );
        }
    }

    let mut annotations_ext = None;
    let mut annotation_layers = Vec::new();
    if let Some(generator) = generator {
        (annotations_ext, annotation_layers) =
            match handle_annotations(&path, annotations_file, generator) {
                Ok((name, layers)) => (Some(name), layers),
                Err(response) => return response,
            };
    }

    let (upl_img_ext, metadata_layers) =
        match handle_image(&mut logger, image_file, &path, &name, &encoder) {
            Ok(layers) => layers,
            Err(response) => return response,
        };

    // Insert into database.
    match crate::db::image::insert(
        parent_id,
        &name,
        &upl_img_ext,
        &upl_img_ext,
        &encoder_name,
        annotations_ext.as_deref(),
        metadata_layers,
        annotation_layers,
    ) {
        Ok(()) => {
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

    match crate::db::general::get_registry() {
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

fn handle_image(
    logger: &mut Logger<'_>,
    file: FieldData<NamedTempFile>,
    path: &std::path::Path,
    name: &str,
    encoder: &Box<dyn Encoder>,
) -> Result<(String, Vec<MetadataLayer>), Response> {
    // Extract image extension from metadata request body.
    let image_filename = file.metadata.file_name.clone();
    let Some(upl_img_ext) = image_filename
        .as_ref()
        .map(std::path::Path::new)
        .and_then(|name| name.extension())
        .and_then(|ext| ext.to_str())
    else {
        return Err(logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "IUI-E00",
            "Uploaded image has no extension.",
            None,
        ));
    };

    // Path where the uploaded image will be stored.
    let upl_img_path = path.join(format!("original-image.{upl_img_ext}"));

    // Path where the encoded image will be stored.
    let enc_img_path = path.join("image.zarr");

    // Path where the thumbnail will be stored.
    let thumbnail_path = path.join("thumbnail.jpeg");

    // Save image to disk.
    match crate::io::save_asset(file.contents, &upl_img_path) {
        Ok(()) => {
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
    match crate::io::try_convert(
        &upl_img_path,
        upl_img_ext,
        &enc_img_path,
        &thumbnail_path,
        encoder,
    ) {
        Ok(metadata) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                &format!("Successfully converted image with name `{name}` to Zarr."),
                None,
            );

            Ok((upl_img_ext.into(), metadata))
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to convert the image with name `{name}` to Zarr."),
                Some(e),
            );

            Err(resp)
        }
    }
}

fn handle_annotations(
    path: &std::path::Path,
    file: Option<FieldData<NamedTempFile>>,
    generator: Box<dyn Generator>,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // If user provides annotations file, translate it and compute buffer geometries.
    // Else generate annotations and compute buffer geometries.
    match file {
        Some(file) => translate_annotations(path, file, &generator),
        None => generate_annotations(path, generator),
    }
}

fn generate_annotations(
    _path: &std::path::Path,
    _generator: Box<dyn Generator>,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // TODO: Generate annotations.
    log::<()>(
        StatusCode::CREATED,
        "No annotations provided. TODO: Generate annotations.",
        None,
    );

    todo!("Generating annotations not supported yet.")
}

fn translate_annotations(
    path: &std::path::Path,
    file: FieldData<NamedTempFile>,
    generator: &Box<dyn Generator>,
) -> Result<(String, Vec<AnnotationLayer>), Response> {
    // Get uploaded annotation file extension from metadata request body.
    let Some(upl_anno_ext) = file
        .metadata
        .file_name
        .as_ref()
        .map(std::path::Path::new)
        .and_then(|path| path.extension())
        .and_then(|ext| ext.to_str())
    else {
        let resp = log::<()>(
            StatusCode::BAD_REQUEST,
            "Failed to retrieve annotation file extension from file metadata.",
            None,
        );

        return Err(resp);
    };

    // Path where the uploaded annotations file will be stored.
    let upl_anno_path = path.join(format!("original-annotations.{upl_anno_ext}"));

    // Save uploaded annotations file to disk.
    match crate::io::save_asset(file.contents, &upl_anno_path) {
        Ok(()) => {
            #[cfg(feature = "log.success")]
            log::<()>(
                StatusCode::CREATED,
                "Successfully saved annotations file to disk.",
                None,
            );
        }
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save annotations file to disk.",
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
                "Failed to translate annotations file with name using the generator.",
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
    let transl_anno_path = path.join("annotations.json");
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

    Ok((upl_anno_ext.to_owned(), annotation_layers))
}
