use crate::{
    constants::{
        ANNOTATIONS_DIRECTORY, LOCAL_DATABASES_PATH, LOCAL_STORES_PATH, MAX_THUMBNAIL_SIZE,
        UPLOADED_DIRECTORY,
    },
    types::messages::TileServerMsg,
};
use anyhow::Result;
use image::RgbImage;
use shared::{
    constants::{TILE_SIZE, TILE_SPLIT_LENGTH},
    traits::Encoder,
    types::{MetadataLayer, Size},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

pub fn create_store_database(store_id: u32) -> Result<String> {
    let path = Path::new(LOCAL_DATABASES_PATH).join(format!("s{store_id}.sqlite"));

    fs::File::create(&path)?;

    // FIXME: Dont prefix with "../".
    Ok(format!(
        "sqlite://../{}",
        path.to_str()
            .ok_or(anyhow::anyhow!("Failed to convert path to string"))?
    ))
}

pub fn create_store(store_id: u32) -> Result<PathBuf> {
    let path = Path::new(LOCAL_STORES_PATH).join(format!("s{store_id}"));
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn create(store_id: u32, image_id: u32) -> Result<PathBuf> {
    let path = Path::new(LOCAL_STORES_PATH)
        .join(format!("s{store_id}"))
        .join(format!("i{image_id}"));

    // Create directory.
    fs::create_dir_all(&path)?;
    fs::create_dir_all(path.join(UPLOADED_DIRECTORY))?;
    fs::create_dir_all(path.join(ANNOTATIONS_DIRECTORY))?;

    Ok(path)
}

pub fn delete(store_id: u32, image_id: u32) -> Result<()> {
    let path = Path::new(LOCAL_STORES_PATH)
        .join(format!("s{store_id}"))
        .join(format!("i{image_id}"));

    // Remove directory.
    fs::remove_dir_all(path)?;

    Ok(())
}

pub fn save_asset(file: NamedTempFile, path: &Path) -> Result<()> {
    file.persist(path)?;

    Ok(())
}

// TODO: Remove encoder hardcode.
pub fn retrieve(path: &Path, level: u32, x: u32, y: u32) -> Result<TileServerMsg> {
    let Some(encoder) = encoders::export::get("OMEZarr") else {
        return Err(anyhow::anyhow!("Could not get encoder."));
    };

    let mut rgb_buffer = vec![0_u8; TILE_SPLIT_LENGTH].into_boxed_slice();
    encoder.retrieve(&mut rgb_buffer, path, level, x, y)?;

    let Some(bmp_buffer) = RgbImage::from_raw(TILE_SIZE, TILE_SIZE, rgb_buffer.into()) else {
        return Err(anyhow::anyhow!("RGB data doesn't fit into image buffer."));
    };

    let jpeg_buffer = turbojpeg::compress_image(&bmp_buffer, 70, turbojpeg::Subsamp::Sub2x2)?;

    // TODO: Fix hardcoding
    Ok(TileServerMsg {
        store_id: 0,
        id: 0,
        level,
        x,
        y,
        buffer: jpeg_buffer.to_vec(),
    })
}

pub fn convert(
    source_path: &Path,
    source_extension: &str,
    destination_path: &Path,
    thumbnail_path: &Path,
    encoder: &Box<dyn Encoder>,
) -> Result<(String, Vec<MetadataLayer>)> {
    // Open the image with a decoder.
    let Some(decoder) = decoders::export::get(source_extension, source_path) else {
        return Err(anyhow::anyhow!("No decoders found for image."));
    };

    match encoder.convert(destination_path, &decoder) {
        Ok(metadata) => {
            // Create thumbnail.
            let larger_dim = metadata[0].width.max(metadata[0].height);

            let thumbnail_buffer = decoder.thumbnail(&Size {
                width: metadata[0].width / larger_dim * MAX_THUMBNAIL_SIZE,
                height: metadata[0].height / larger_dim * MAX_THUMBNAIL_SIZE,
            })?;

            // Convert thumbnail buffer to JPEG.
            let thumbnail_jpeg =
                turbojpeg::compress_image(&thumbnail_buffer, 70, turbojpeg::Subsamp::Sub2x2)?
                    .to_vec();

            // Save thumbnail to disk.
            fs::write(thumbnail_path, thumbnail_jpeg)?;

            Ok((decoder.name().to_string(), metadata))
        }
        Err(e) => Err(anyhow::anyhow!("Failed to encode image: {e}")),
    }
}
