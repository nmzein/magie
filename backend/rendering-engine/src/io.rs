use crate::types::TileRequest;
use anyhow::Result;
use image::RgbImage;
use shared::traits::Encoder;
use shared::{
    constants::TILE_SIZE,
    structs::{MetadataLayer, Size},
};
use std::path::Path;
use tempfile::NamedTempFile;
use tokio::fs;

static THUMBNAIL_WIDTH: u32 = 256;
static THUMBNAIL_HEIGHT: u32 = 128;

pub async fn create(path: &Path) -> Result<()> {
    // Create directory.
    fs::create_dir_all(&path).await?;

    Ok(())
}

pub async fn delete(path: &Path) -> Result<()> {
    // Remove directory.
    fs::remove_dir_all(path).await?;

    Ok(())
}

pub async fn r#move(source_path: &Path, destination_base: &Path) -> Result<()> {
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

    fs::rename(source_path, destination_path).await?;

    Ok(())
}

pub fn save_asset(file: NamedTempFile, path: &Path) -> Result<()> {
    file.persist(path)?;

    Ok(())
}

pub fn retrieve(path: &Path, tile_request: &TileRequest) -> Result<Vec<u8>> {
    // TODO: Remove hardcode
    let encoder = encoders::export::get("OMEZarr").unwrap();

    let raw_buffer = encoder.retrieve(path, tile_request.level, tile_request.x, tile_request.y)?;

    #[cfg(feature = "time")]
    let start = std::time::Instant::now();

    let Some(image_buffer) = RgbImage::from_raw(TILE_SIZE, TILE_SIZE, raw_buffer) else {
        return Err(anyhow::anyhow!(
            "Could not convert tile Vec<u8> to ImageBuffer."
        ));
    };

    #[cfg(feature = "time")]
    println!("Convert to buffer took: {:?}", start.elapsed());

    #[cfg(feature = "time")]
    let start = std::time::Instant::now();

    // #2 Bottleneck
    let jpeg_tile = turbojpeg::compress_image(&image_buffer, 70, turbojpeg::Subsamp::Sub2x2)?;

    #[cfg(feature = "time")]
    println!("Convert to jpeg took: {:?}", start.elapsed());

    #[cfg(feature = "time")]
    let start = std::time::Instant::now();

    // Prepend tile position and level
    // (will be in this form [level, x, y, tile...])
    let res = [
        tile_request.level.to_be_bytes().as_slice(),
        tile_request.x.to_be_bytes().as_slice(),
        tile_request.y.to_be_bytes().as_slice(),
        jpeg_tile.to_vec().as_slice(),
    ]
    .concat();

    #[cfg(feature = "time")]
    println!("Final insert took: {:?}", start.elapsed());

    Ok(res)
}

pub async fn try_convert(
    source_path: &Path,
    source_extension: &str,
    destination_path: &Path,
    thumbnail_path: &Path,
    encoder: Box<dyn Encoder>,
) -> Result<Vec<MetadataLayer>> {
    let decoders = decoders::export::get(source_extension);
    if decoders.is_empty() {
        return Err(anyhow::anyhow!("No decoders found for image."));
    }

    for mut decoder in decoders {
        // Open the image with the decoder.
        decoder.open(source_path)?;

        // Create thumbnail.
        let thumbnail_buffer = decoder.thumbnail(&Size {
            width: THUMBNAIL_WIDTH,
            height: THUMBNAIL_HEIGHT,
        })?;

        // If successful, return early, otherwise log error and continue.
        match encoder.convert(destination_path, decoder) {
            Ok(metadata) => {
                // Convert thumbnail buffer to JPEG.
                let thumbnail_jpeg =
                    turbojpeg::compress_image(&thumbnail_buffer, 70, turbojpeg::Subsamp::Sub2x2)?
                        .to_vec();

                // Save thumbnail to disk.
                fs::write(thumbnail_path, thumbnail_jpeg).await?;

                return Ok(metadata);
            }
            Err(e) => {
                eprintln!("Error <Decoders>: Decoder failed to convert image.");
                eprintln!("Details: {e:?}");
                eprintln!();
            }
        }
    }

    // None of the decoders were successful.
    Err(anyhow::anyhow!("All decoders failed to convert image."))
}
