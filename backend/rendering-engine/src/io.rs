use crate::types::TileRequest;
use anyhow::Result;
use image::RgbImage;
use shared::constants::TILE_SPLIT_LENGTH;
use shared::traits::Encoder;
use shared::{
    constants::TILE_SIZE,
    structs::{MetadataLayer, Size},
};
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

static THUMBNAIL_WIDTH: u32 = 256;
static THUMBNAIL_HEIGHT: u32 = 128;

pub fn create(path: &Path) -> Result<()> {
    // Create directory.
    fs::create_dir_all(path)?;

    Ok(())
}

pub fn delete(path: &Path) -> Result<()> {
    // Remove directory.
    fs::remove_dir_all(path)?;

    Ok(())
}

pub fn r#move(source_path: &Path, destination_base: &Path) -> Result<()> {
    // Extract the last segment of the source path
    let Some(last_segment) = source_path.file_name() else {
        return Err(anyhow::anyhow!(
            "Could not extract last segment from source path."
        ));
    };

    let destination_path = destination_base.join(last_segment);

    // Check if the destination path already exists.
    if destination_path.exists() {
        return Err(anyhow::anyhow!(
            "Directory with the same name already exists at the destination."
        ));
    }

    fs::rename(source_path, destination_path)?;

    Ok(())
}

pub fn save_asset(file: NamedTempFile, path: &Path) -> Result<()> {
    file.persist(path)?;

    Ok(())
}

// TODO: Remove encoder hardcode.
pub fn retrieve(path: &Path, &TileRequest { id: _, x, y, level }: &TileRequest) -> Result<Vec<u8>> {
    let Some(encoder) = encoders::export::get("OMEZarr") else {
        return Err(anyhow::anyhow!("Could not get encoder."));
    };

    let mut rgb_buffer = vec![0_u8; TILE_SPLIT_LENGTH].into_boxed_slice();
    encoder.retrieve(&mut rgb_buffer, path, level, x, y)?;

    let Some(bmp_buffer) = RgbImage::from_raw(TILE_SIZE, TILE_SIZE, rgb_buffer.into()) else {
        return Err(anyhow::anyhow!("RGB data doesn't fit into image buffer."));
    };

    let jpeg_buffer = turbojpeg::compress_image(&bmp_buffer, 70, turbojpeg::Subsamp::Sub2x2)?;

    // Prepend tile level and position (will be in this form [level, x, y, jpeg]).
    let res = [
        level.to_be_bytes().as_slice(),
        x.to_be_bytes().as_slice(),
        y.to_be_bytes().as_slice(),
        jpeg_buffer.as_ref(),
    ]
    .concat();

    Ok(res)
}

pub fn try_convert(
    source_path: &Path,
    source_extension: &str,
    destination_path: &Path,
    thumbnail_path: &Path,
    encoder: &Box<dyn Encoder>,
) -> Result<Vec<MetadataLayer>> {
    // Open the image with a decoder.
    let Some(decoder) = decoders::export::get(source_extension, source_path) else {
        return Err(anyhow::anyhow!("No decoders found for image."));
    };

    match encoder.convert(destination_path, &decoder) {
        Ok(metadata) => {
            // Create thumbnail.
            let thumbnail_buffer = decoder.thumbnail(&Size {
                width: THUMBNAIL_WIDTH,
                height: THUMBNAIL_HEIGHT,
            })?;

            // Convert thumbnail buffer to JPEG.
            let thumbnail_jpeg =
                turbojpeg::compress_image(&thumbnail_buffer, 70, turbojpeg::Subsamp::Sub2x2)?
                    .to_vec();

            // Save thumbnail to disk.
            fs::write(thumbnail_path, thumbnail_jpeg)?;

            Ok(metadata)
        }
        Err(e) => Err(anyhow::anyhow!("Failed to encode image: {e}")),
    }
}
