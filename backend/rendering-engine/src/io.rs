use crate::types::TileRequest;
use anyhow::Result;
use image::RgbImage;
use shared::{constants::*, structs::MetadataLayer};
#[cfg(feature = "time")]
use std::time::Instant;
use std::{path::Path, sync::Arc};
use tempfile::NamedTempFile;
use tokio::fs;
use zarrs::{array::Array, array_subset::ArraySubset, storage::store::FilesystemStore};

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

pub fn interleave<'a>(channels: &[u8], tile: &'a mut Vec<u8>) -> &'a [u8] {
    tile.clear();
    tile.reserve(TILE_SPLIT_LENGTH);

    let rs = &channels[..TILE_LENGTH];
    let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
    let bs = &channels[TILE_LENGTH * 2..];

    tile.extend(
        rs.iter()
            .zip(gs)
            .zip(bs)
            .flat_map(|((&r, &g), &b)| [r, g, b]),
    );

    tile
}

pub async fn retrieve(path: &Path, tile_request: &TileRequest) -> Result<Vec<u8>> {
    #[cfg(feature = "time")]
    let start = Instant::now();

    // TODO: Move from here.
    let store = Arc::new(FilesystemStore::new(path)?);
    let array = Arc::new(Array::open(
        store,
        &format!("{GROUP_PATH}/{}", tile_request.level),
    )?);

    let x: u64 = tile_request.x.into();
    let y: u64 = tile_request.y.into();
    let level = tile_request.level;

    #[cfg(feature = "time")]
    let start = time("Tile initialisation", level, x, y, start);

    // Retrieve tile for each RGB channel.
    let channels = array.retrieve_chunks_elements(&ArraySubset::new_with_start_end_inc(
        vec![0, 0, 0, y, x],
        vec![0, 2, 0, y, x],
    )?)?;

    #[cfg(feature = "time")]
    let start = time("Retrieving tile", level, x, y, start);

    // Interleave RGB channels.
    let mut tile = Vec::with_capacity(TILE_LENGTH * 3);
    interleave(&channels, &mut tile);

    // TODO: To here. Inside encoders.

    #[cfg(feature = "time")]
    let start = time("Interleaving RGB channels", level, x, y, start);

    let Some(image_tile) = RgbImage::from_raw(TILE_SIZE, TILE_SIZE, tile) else {
        return Err(anyhow::anyhow!(
            "Could not convert tile Vec<u8> to ImageBuffer."
        ));
    };

    let mut jpeg_tile =
        turbojpeg::compress_image(&image_tile, 70, turbojpeg::Subsamp::Sub2x2)?.to_vec();

    #[cfg(feature = "time")]
    time("Encoding tile to JPEG", level, x, y, start);

    // Prepend tile position and level
    // (will be in this form [level, x, y, tile...])
    // ! FIX: x, y can be > u8.
    jpeg_tile.insert(0, y as u8);
    jpeg_tile.insert(0, x as u8);
    jpeg_tile.insert(0, level as u8);

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

    let Some(encoder) = encoders::export::get("OMEZarr") else {
        return Err(anyhow::anyhow!("Encoder not found."));
    };

    for decoder in decoders {
        // If successful, return early, otherwise log error and continue.
        match encoder.convert(&upl_img_path, &enc_img_path, decoder) {
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
