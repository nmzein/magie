use anyhow::Result;
use image::RgbImage;
use shared::{
    constants::{ANNOTATIONS_DIRECTORY, TILE_SIZE, TILE_SPLIT_LENGTH, UPLOADED_DIRECTORY},
    traits::Encoder,
    types::{MetadataLayer, Size},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

static THUMBNAIL_WIDTH: u32 = 256;
static THUMBNAIL_HEIGHT: u32 = 128;

static LOCAL_STORE_BASE_PATH: &str = "../stores/";

pub fn create(store_id: u32, image_id: u32) -> Result<PathBuf> {
    let path = Path::new(LOCAL_STORE_BASE_PATH)
        .join(format!("s{store_id}"))
        .join(format!("i{image_id}"));

    // Create directory.
    fs::create_dir_all(&path)?;
    fs::create_dir_all(path.join(UPLOADED_DIRECTORY))?;
    fs::create_dir_all(path.join(ANNOTATIONS_DIRECTORY))?;

    Ok(path)
}

pub fn delete(path: &Path) -> Result<()> {
    // Remove directory.
    fs::remove_dir_all(path)?;

    Ok(())
}

pub fn save_asset(file: NamedTempFile, path: &Path) -> Result<()> {
    file.persist(path)?;

    Ok(())
}

// TODO: Remove encoder hardcode.
pub fn retrieve(path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>> {
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
