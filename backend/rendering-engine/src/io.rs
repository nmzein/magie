use crate::structs::{Metadata, TileRequest};
use anyhow::Result;
use image::RgbImage;
use shared::{
    structs::{Address, Region, Size},
    traits::Decoder,
};
#[cfg(feature = "time")]
use std::time::Instant;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tempfile::NamedTempFile;
use tokio::fs;
use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

static TILE_SIZE: u32 = 1024;
static TILE_LENGTH: usize = (TILE_SIZE * TILE_SIZE) as usize;
static TILE_SPLIT_LENGTH: usize = (TILE_SIZE * TILE_SIZE * 3) as usize;
static RGB_CHANNELS: u64 = 3;
static GROUP_PATH: &str = "/group";
static STORE_PATH: &str = "store";

pub async fn create(directory_path: &PathBuf) -> Result<PathBuf> {
    let directory_path = PathBuf::from(STORE_PATH).join(directory_path);

    // Create directory.
    fs::create_dir_all(&directory_path).await?;

    Ok(directory_path)
}

pub async fn delete(directory_path: &PathBuf) -> Result<()> {
    let directory_path = PathBuf::from(STORE_PATH).join(directory_path);

    // Remove directory.
    fs::remove_dir_all(directory_path).await?;

    Ok(())
}

pub async fn save_asset(file: NamedTempFile, path: &PathBuf) -> Result<()> {
    let path = PathBuf::from(STORE_PATH).join(path);

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

pub async fn retrieve(store_path: &PathBuf, tile_request: &TileRequest) -> Result<Vec<u8>> {
    #[cfg(feature = "time")]
    let start = Instant::now();

    let store_path = PathBuf::from(STORE_PATH).join(store_path);
    let store = Arc::new(FilesystemStore::new(store_path)?);
    let array = Arc::new(Array::new(
        store,
        &format!("{GROUP_PATH}/{}", tile_request.level),
    )?);

    let x: u64 = tile_request.x.into();
    let y: u64 = tile_request.y.into();
    let level = tile_request.level;

    #[cfg(feature = "time")]
    let start = time("Tile initialisation", level, x, y, start);

    // Retrieve tile for each RGB channel.
    let channels = array.retrieve_chunks(&ArraySubset::new_with_start_end_inc(
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

pub async fn convert(
    image_path: &PathBuf,
    store_path: &PathBuf,
    decoders: Arc<Mutex<Vec<Box<dyn Decoder>>>>,
) -> Result<Vec<Metadata>> {
    let image_path = PathBuf::from(STORE_PATH).join(image_path);
    let store_path = PathBuf::from(STORE_PATH).join(store_path);

    let decoder_lock = decoders.lock().unwrap();
    let decoder = decoder_lock
        .iter()
        .find(|decoder| {
            decoder
                .supported_extensions()
                .contains(&image_path.extension().unwrap().to_str().unwrap())
        })
        .ok_or(anyhow::anyhow!("No decoder found for image."))?;

    // One store per image.
    let store = Arc::new(FilesystemStore::new(store_path)?);
    // One group per image.
    let group = GroupBuilder::new().build(store.clone(), GROUP_PATH)?;
    // Write group metadata to store.
    // ! Remove group and make it so better adheres to OME-ZARR.
    group.store_metadata()?;

    let levels = decoder.get_level_count(&image_path)?;
    if levels == 0 {
        return Err(anyhow::anyhow!("Image has no levels."));
    }
    let (level_0_width, level_0_height) = decoder.get_level_dimensions(&image_path, 0)?;

    let mut metadata: Vec<Metadata> = Vec::new();

    for level in 0..levels {
        // Get image dimensions.
        let (width, height) = decoder.get_level_dimensions(&image_path, level)?;

        // Calculate number of tiles per row and column.
        let cols = width.div_ceil(TILE_SIZE);
        let rows = height.div_ceil(TILE_SIZE);

        // ! Check these conversions.
        let width_ratio = (level_0_width as f64 / width as f64) as u32;
        let height_ratio = (level_0_height as f64 / height as f64) as u32;

        // One array per image level.
        let array_path = format!("{}/{}", GROUP_PATH, level);

        let array = ArrayBuilder::new(
            // Define image shape.
            vec![0, RGB_CHANNELS, 0, height.into(), width.into()],
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
                        &image_path,
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
                    .collect();

                #[cfg(feature = "time")]
                let start = time(
                    "Reading and rearranging tile",
                    level,
                    x.into(),
                    y.into(),
                    start,
                );

                array.store_chunks(
                    &ArraySubset::new_with_start_end_inc(
                        vec![0, 0, 0, y.into(), x.into()],
                        vec![0, 2, 0, y.into(), x.into()],
                    )?,
                    tile,
                )?;

                #[cfg(feature = "time")]
                time("Storing tile", level, x.into(), y.into(), start);
            }
        }

        metadata.push(Metadata {
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
