use crate::api::prelude::*;
use crate::constants::{
    ANNOTATIONS_PATH_PREFIX, IMAGE_NAME, THUMBNAIL_NAME, TRANSLATED_ANNOTATIONS_PATH,
    UPLOADED_ANNOTATIONS_PATH, UPLOADED_IMAGE_PATH,
};
use anyhow::anyhow;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use shared::{
    traits::{Encoder, Generator},
    types::{AnnotationLayer, MetadataLayer},
};
use std::{fs, process::Command};
use tempfile::NamedTempFile;

#[derive(Deserialize)]
pub struct PathParams {
    store_id: u32,
    parent_id: u32,
    name: String,
}

#[derive(TryFromMultipart)]
pub struct Multipart {
    decoder: Option<String>,
    encoder: String,
    generator: Option<String>,
    #[form_data(limit = "unlimited")]
    image_file: FieldData<NamedTempFile>,
    #[form_data(limit = "unlimited")]
    annotations_file: Option<FieldData<NamedTempFile>>,
}

fn extract_extension(filename: &Option<String>) -> Option<String> {
    let extension = filename
        .as_ref()
        .map(std::path::Path::new)
        .and_then(|name| name.extension())
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_owned());

    extension
}

// TODO: Make agnostic to file type and split up based on generate/upload.
// TODO: Handle half failed states.
// TODO: Perform checks on files before saving them to avoid malware.
// TODO: Sanitise file name.
pub async fn upload(
    Extension(dbm): Extension<Arc<DatabaseManager>>,
    Extension(mut logger): Extension<Logger<'_>>,
    Path(PathParams {
        store_id,
        parent_id,
        name,
    }): Path<PathParams>,
    TypedMultipart(Multipart {
        decoder,
        encoder,
        generator,
        image_file,
        annotations_file,
    }): TypedMultipart<Multipart>,
) -> Response {
    // Extract image extension from metadata request body.
    let Some(uploaded_image_extension) = extract_extension(&image_file.metadata.file_name.clone())
    else {
        return logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "IU-E00",
            "Uploaded image has no extension.",
            None,
        );
    };

    // Extract annotations extension from metadata request body.
    let uploaded_annotations_extension = annotations_file
        .as_ref()
        .and_then(|f| extract_extension(&f.metadata.file_name.clone()));

    if annotations_file.is_some() && uploaded_annotations_extension.is_none() {
        return logger.error(
            StatusCode::BAD_REQUEST,
            Error::RequestIntegrity,
            "IU-E01",
            "Uploaded annotations file has no extension.",
            None,
        );
    }

    // Get the encoder object that will be used to encode the image to Zarr.
    let encoder_object = match encoders::export::get(encoder.as_str()) {
        Some(encoder) => {
            logger.report(Check::ResourceExistence, "Encoder found.");
            encoder
        }
        None => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "IU-E02",
                "Encoder could not be found.",
                None,
            );
        }
    };

    // Get the generator object that will be used to translate or generate annotations.
    let generator_object = match generator.as_ref().map(|g| generators::export::get(g)) {
        Some(Some(generator)) => {
            logger.report(Check::ResourceExistence, "Generator found.");
            Some(generator)
        }
        Some(None) => {
            return logger.error(
                StatusCode::NOT_FOUND,
                Error::ResourceExistence,
                "IU-E03",
                "Generator could not be found.",
                None,
            );
        }
        None => None,
    };

    let Ok(image_id) = crate::db::counter::counter(&dbm, store_id) else {
        return logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResourceCreation,
            "IU-E04",
            "Failed to generate image ID.",
            None,
        );
    };

    // Create a directory in local store for the image.
    let path = match crate::io::create(store_id, image_id) {
        Ok(path) => path,
        Err(e) => {
            return logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IU-E05",
                "Failed to create a directory for asset.",
                Some(e),
            );
        }
    };

    let mut annotation_layers = Vec::new();
    if let Some(generator_object) = generator_object {
        if let Some(uploaded_annotations_extension) = &uploaded_annotations_extension {
            annotation_layers = match handle_annotations(
                &mut logger,
                &path,
                uploaded_annotations_extension,
                annotations_file,
                generator_object,
            ) {
                Ok(layers) => layers,
                Err(response) => return response,
            };
        }
    };

    let (decoder, metadata_layers) = match handle_image(
        &mut logger,
        image_file,
        &path,
        &uploaded_image_extension,
        &encoder_object,
    ) {
        Ok(layers) => layers,
        Err(response) => return response,
    };

    match crate::db::image::insert(
        &dbm,
        store_id,
        image_id,
        parent_id,
        &name,
        &decoder,
        &encoder,
        generator.as_deref(),
        &uploaded_image_extension,
        uploaded_annotations_extension.as_deref(),
        metadata_layers,
        annotation_layers,
    ) {
        Ok(()) => {
            logger.report(
                Check::ResourceConflict,
                "Successfully saved asset to database.",
            );
        }
        Err(e) => {
            return logger.error(
                StatusCode::CONFLICT,
                Error::ResourceCreation,
                "IU-E06",
                "Failed to save asset to database.",
                Some(e),
            );
        }
    }

    logger.success(StatusCode::CREATED, "Successfully uploaded assets.");

    (StatusCode::OK).into_response()
}

fn handle_image(
    logger: &mut Logger<'_>,
    file: FieldData<NamedTempFile>,
    path: &std::path::Path,
    extension: &str,
    encoder: &Box<dyn Encoder>,
) -> Result<(String, Vec<MetadataLayer>), Response> {
    // Path where the uploaded image will be stored.
    let uploaded_image_path = path.join(UPLOADED_IMAGE_PATH);

    // Path where the encoded image will be stored.
    let final_image_path = path.join(IMAGE_NAME);

    // Path where the thumbnail will be stored.
    let thumbnail_path = path.join(THUMBNAIL_NAME);

    // Save image to disk.
    match crate::io::save_asset(file.contents, &uploaded_image_path) {
        Ok(()) => logger.log("Successfully saved image to disk."),
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IUI-E00",
                "Failed to save image to disk.",
                Some(e),
            ));
        }
    }

    // Encode image to Zarr derivative format.
    match crate::io::convert(
        &uploaded_image_path,
        extension,
        &final_image_path,
        &thumbnail_path,
        encoder,
    ) {
        Ok((decoder, metadata)) => {
            logger.log("Successfully converted image to Zarr.");
            Ok((decoder, metadata))
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IUI-E01",
                "Failed to convert image to Zarr.",
                Some(e),
            ));
        }
    }
}

fn handle_annotations(
    logger: &mut Logger<'_>,
    path: &std::path::Path,
    extension: &str,
    file: Option<FieldData<NamedTempFile>>,
    generator: Box<dyn Generator>,
) -> Result<Vec<AnnotationLayer>, Response> {
    // If user provides annotations file, translate it and compute buffer geometries.
    // Else generate annotations and compute buffer geometries.
    match file {
        Some(file) => translate_annotations(logger, path, extension, file, &generator),
        None => generate_annotations(logger, path, extension, generator),
    }
}

// TODO: Generate annotations.
fn generate_annotations(
    _logger: &mut Logger<'_>,
    _path: &std::path::Path,
    _extension: &str,
    _generator: Box<dyn Generator>,
) -> Result<Vec<AnnotationLayer>, Response> {
    todo!("Generating annotations not supported yet.")
}

fn translate_annotations(
    logger: &mut Logger<'_>,
    path: &std::path::Path,
    extension: &str,
    file: FieldData<NamedTempFile>,
    generator: &Box<dyn Generator>,
) -> Result<Vec<AnnotationLayer>, Response> {
    // Path where the uploaded annotations file will be stored.
    let uploaded_annotations_path = path.join(format!("{UPLOADED_ANNOTATIONS_PATH}.{extension}"));

    // Path where intermediate Three.js buffer geometries will be stored.
    let translated_annotations_path = path.join(TRANSLATED_ANNOTATIONS_PATH);

    // Path where the GLB annotations will be stored.
    let final_annotations_path = path.join(ANNOTATIONS_PATH_PREFIX);

    // Save uploaded annotations file to disk.
    match crate::io::save_asset(file.contents, &uploaded_annotations_path) {
        Ok(()) => logger.log("Successfully saved annotations file to disk."),
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IUTA-E00",
                "Failed to save annotations file to disk.",
                Some(e),
            ));
        }
    }

    // Translate annotations.
    let annotation_layers = match generator.translate(&uploaded_annotations_path) {
        Ok(layers) => {
            logger.log("Successfully translated annotations file.");
            layers
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IUTA-E01",
                "Failed to translate annotations file.",
                Some(e),
            ));
        }
    };

    // TODO: Use capnproto rather than storing an intermediate translated annotations json file.
    // TODO: Or, try writing own buffer geometry creator/gltf lib in Rust.
    // Serialize annotation layers.
    let Ok(serialized_annotation_layers) = serde_json::to_string(&annotation_layers) else {
        return Err(logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResourceCreation,
            "IUTA-E02",
            "Failed to serialise annotations file.",
            None,
        ));
    };

    fs::write(&translated_annotations_path, serialized_annotation_layers).map_err(|e| {
        return logger.error(
            StatusCode::INTERNAL_SERVER_ERROR,
            Error::ResourceCreation,
            "IUTA-E03",
            "Failed to save translated annotations file to disk.",
            Some(e.into()),
        );
    })?;

    // Compute annotation positions and normals.
    match Command::new("node")
        .arg("--max-old-space-size=4096")
        .arg("./geometry-computer/index.js")
        .arg(translated_annotations_path)
        .arg(final_annotations_path)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                logger.log(
                    "Successfully computed annotation positions and normals and saved to disk.",
                );
            } else {
                let e = String::from_utf8(output.stderr).map_err(|e| {
                    return logger.error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Error::ResourceCreation,
                        "IUTA-E04",
                        "Failed to convert stderr to string.",
                        Some(e.into()),
                    );
                })?;

                return Err(logger.error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::ResourceCreation,
                    "IUTA-E05",
                    "Failed to compute annotation positions and normals.",
                    Some(anyhow!(e)),
                ));
            }
        }
        Err(e) => {
            return Err(logger.error(
                StatusCode::INTERNAL_SERVER_ERROR,
                Error::ResourceCreation,
                "IUTA-E06",
                "Failed to run geometry computation.",
                Some(e.into()),
            ));
        }
    }

    Ok(annotation_layers)
}
