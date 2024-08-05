use crate::types::{MetadataLayer, TileRequest};
use anyhow::Result;
use image::RgbImage;
use shared::{
    structs::{Address, Region, Size},
    traits::Decoder,
};
#[cfg(feature = "time")]
use std::time::Instant;
use std::{path::PathBuf, sync::Arc};
use tempfile::NamedTempFile;
use tokio::fs;
use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

static RGB_CHANNELS: u32 = 3;
static TILE_SIZE: u32 = 1024;
static TILE_LENGTH: usize = (TILE_SIZE * TILE_SIZE) as usize;
static TILE_SPLIT_LENGTH: usize = (TILE_SIZE * TILE_SIZE * RGB_CHANNELS) as usize;
static GROUP_PATH: &str = "/group";

pub async fn create(path: &PathBuf) -> Result<()> {
    // Create directory.
    fs::create_dir_all(&path).await?;

    Ok(())
}

pub async fn delete(path: &PathBuf) -> Result<()> {
    // Remove directory.
    fs::remove_dir_all(path).await?;

    Ok(())
}

pub async fn r#move(source_path: &PathBuf, destination_base: &PathBuf) -> Result<()> {
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

pub async fn save_asset(file: NamedTempFile, path: &PathBuf) -> Result<()> {
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

pub async fn retrieve(path: &PathBuf, tile_request: &TileRequest) -> Result<Vec<u8>> {
    #[cfg(feature = "time")]
    let start = Instant::now();

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

pub async fn convert(upl_img_path: &PathBuf, enc_img_path: &PathBuf) -> Result<Vec<MetadataLayer>> {
    let Some(extension) = upl_img_path.extension().and_then(|ext| ext.to_str()) else {
        return Err(anyhow::anyhow!("Image has no extension."));
    };

    let decoders = decoders::export::get(extension);
    if decoders.is_empty() {
        return Err(anyhow::anyhow!("No decoders found for image."));
    }

    for decoder in decoders {
        // If successful, return early, otherwise log error and continue.
        match try_convert(&upl_img_path, &enc_img_path, decoder).await {
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

pub async fn try_convert(
    upl_img_path: &PathBuf,
    enc_img_path: &PathBuf,
    decoder: Box<dyn Decoder>,
) -> Result<Vec<MetadataLayer>> {
    // One store per image.
    let store = Arc::new(FilesystemStore::new(enc_img_path)?);
    // One group per image.
    let group = GroupBuilder::new().build(store.clone(), GROUP_PATH)?;
    // Write group metadata to store.
    // ! Remove group and make it so better adheres to OME-ZARR.
    group.store_metadata()?;

    let levels = decoder.get_level_count(&upl_img_path)?;
    if levels == 0 {
        return Err(anyhow::anyhow!("Image has no levels."));
    }
    let (level_0_width, level_0_height) = decoder.get_level_dimensions(&upl_img_path, 0)?;

    let mut metadata: Vec<MetadataLayer> = Vec::new();

    for level in 0..levels {
        // Get image dimensions.
        let (width, height) = decoder.get_level_dimensions(&upl_img_path, level)?;

        // Calculate number of tiles per row and column.
        let cols = width.div_ceil(TILE_SIZE);
        let rows = height.div_ceil(TILE_SIZE);

        // ! Loses accuracy.
        let width_ratio = (level_0_width as f32 / width as f32) as u32;
        let height_ratio = (level_0_height as f32 / height as f32) as u32;

        // One array per image level.
        let array_path = format!("{}/{}", GROUP_PATH, level);

        let array = ArrayBuilder::new(
            // Define image shape.
            vec![0, RGB_CHANNELS.into(), 0, height.into(), width.into()],
            // Define data type.
            DataType::UInt8,
            // Define tile size.
            vec![1, 1, 1, TILE_SIZE.into(), TILE_SIZE.into()].try_into()?,
            // Define initial fill value.
            FillValue::from(41u8),
        )
        // Define compression algorithm and strength.
        .bytes_to_bytes_codecs(vec![
            #[cfg(feature = "lz4")]
            Box::new(codec::Lz4Codec::new(9)?),
        ])
        // Define dimension names - time, RGB channel, z, y, x axis.
        .dimension_names(vec!["t", "c", "z", "y", "x"].into())
        .build(store.clone(), &array_path)?;

        // Write array metadata to store.
        array.store_metadata()?;

        // Write tile data.
        for y in 0..rows {
            for x in 0..cols {
                #[cfg(feature = "time")]
                let start = Instant::now();

                // Rearrange tile from [R,G,B,R,G,B] to [R,R,G,G,B,B].
                let tile = decoder
                    .read_region(
                        &upl_img_path,
                        &Region {
                            size: Size {
                                width: TILE_SIZE,
                                height: TILE_SIZE,
                            },
                            level: level,
                            address: Address {
                                x: (x * TILE_SIZE * width_ratio),
                                y: (y * TILE_SIZE * height_ratio),
                            },
                        },
                    )?
                    .chunks(3)
                    .fold(
                        vec![Vec::new(), Vec::new(), Vec::new()],
                        |mut acc, chunk| {
                            acc[0].push(chunk[0]);
                            acc[1].push(chunk[1]);
                            acc[2].push(chunk[2]);
                            acc
                        },
                    )
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>();

                #[cfg(feature = "time")]
                let start = time(
                    "Reading and rearranging tile",
                    level,
                    x.into(),
                    y.into(),
                    start,
                );

                array.store_chunks_elements(
                    &ArraySubset::new_with_start_end_inc(
                        vec![0, 0, 0, y.into(), x.into()],
                        vec![0, 2, 0, y.into(), x.into()],
                    )?,
                    &tile,
                )?;

                #[cfg(feature = "time")]
                time("Storing tile", level, x.into(), y.into(), start);
            }
        }

        metadata.push(MetadataLayer {
            level,
            cols,
            rows,
            width,
            height,
        });
    }

    Ok(metadata)
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
