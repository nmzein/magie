use crate::types::TileRequest;
use anyhow::Result;
use image::RgbImage;
use shared::{constants::*, structs::MetadataLayer};
use std::path::Path;
#[cfg(feature = "time")]
use std::time::Instant;
use tempfile::NamedTempFile;
use tokio::fs;

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
    let last_segment = match source_path.file_name() {
        Some(name) => name,
        None => {
            return Err(anyhow::anyhow!(
                "Could not extract last segment from source path."
            ))
        }
    };

    let destination_path = destination_base.join(last_segment);

    let _ = fs::rename(source_path, destination_path).await?;

    return Ok(());
}

pub async fn save_asset(file: NamedTempFile, path: &Path) -> Result<()> {
    file.persist(path)?;

    Ok(())
}

pub async fn retrieve(path: &Path, tile_request: &TileRequest) -> Result<Vec<u8>> {
    #[cfg(feature = "time")]
    let start = Instant::now();

    let raw_buffer = encoders::export::retrieve(
        "OMEZarr",
        path,
        tile_request.level,
        tile_request.x,
        tile_request.y,
    )?;

    #[cfg(feature = "time")]
    let start = time("Interleaving RGB channels", level, x, y, start);

    let Some(image_buffer) = RgbImage::from_raw(TILE_SIZE, TILE_SIZE, raw_buffer) else {
        return Err(anyhow::anyhow!(
            "Could not convert tile Vec<u8> to ImageBuffer."
        ));
    };

    let mut jpeg_tile =
        turbojpeg::compress_image(&image_buffer, 70, turbojpeg::Subsamp::Sub2x2)?.to_vec();

    #[cfg(feature = "time")]
    time("Encoding tile to JPEG", level, x, y, start);

    // Prepend tile position and level
    // (will be in this form [level, x, y, tile...])
    // ! FIX: x, y can be > u8.
    jpeg_tile.insert(0, tile_request.y as u8);
    jpeg_tile.insert(0, tile_request.x as u8);
    jpeg_tile.insert(0, tile_request.level as u8);

    Ok(jpeg_tile)
}

pub async fn convert(upl_img_path: &Path, enc_img_path: &Path) -> Result<Vec<MetadataLayer>> {
    let Some(extension) = upl_img_path.extension().and_then(|ext| ext.to_str()) else {
        return Err(anyhow::anyhow!("Image has no extension."));
    };

    let decoders = decoders::export::get(extension);
    if decoders.is_empty() {
        return Err(anyhow::anyhow!("No decoders found for image."));
    }

    for decoder in decoders {
        // If successful, return early, otherwise log error and continue.
        match encoders::export::convert("OMEZarr", &upl_img_path, &enc_img_path, decoder) {
            Ok(metadata) => return Ok(metadata),
            Err(e) => {
                eprintln!("Error <Decoders>: Decoder failed to convert image.");
                eprintln!("Details: {:?}", e);
                eprintln!();
            }
        }
    }

    // None of the decoders were successful.
    Err(anyhow::anyhow!("All decoders failed to convert image."))
}

#[cfg(feature = "time")]
fn time(message: &str, level: u32, x: u64, y: u64, start: Instant) -> Instant {
    println!(
        "<{}:({}, {})>: {} took: {:?}",
        level,
        x,
        y,
        message,
        start.elapsed()
    );
    Instant::now()
}
