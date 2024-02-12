use crate::structs::{Address, AnnotationLayer, Metadata, Region, Selection, Size};
use crate::traits::Decoder;
use crate::mpsc::Sender;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use axum::extract::ws::Message;
use image::codecs::jpeg::JpegEncoder;
use tokio::{fs, task::JoinHandle};
use zarrs::{
    array::{chunk_grid::ChunkGridTraits, Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};
use futures::future::join_all;

#[cfg(feature = "time")]
use std::time::Instant;

static CHUNK_SIZE: u32 = 1024;
static CHUNK_LENGTH: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;
static RGB_CHANNELS: u64 = 3;
static GROUP_PATH: &str = "/group";
static STORE_PATH: &str = "../store";

pub async fn create(image_name: &str) -> Result<PathBuf> {
    let directory_path = PathBuf::from(STORE_PATH).join(image_name);

    // Create directory.
    fs::create_dir_all(&directory_path).await?;

    Ok(directory_path)
}

pub async fn remove(image_name: &str) -> Result<()> {
    let directory_path = PathBuf::from(STORE_PATH).join(image_name);

    // Remove directory.
    fs::remove_dir_all(directory_path).await?;

    Ok(())
}

// TODO: Run annotation generator translation interface.
pub async fn annotations(annotations_path: &str) -> Result<Vec<AnnotationLayer>> {
    Ok(crate::generators::tiatoolbox::read_annotations(annotations_path).await?)
}

pub async fn retrieve(
    store_path: &PathBuf,
    selection: Selection,
    sender: Sender<Message>,
) -> Result<()> {
    let store = Arc::new(FilesystemStore::new(store_path)?);
    let array = Arc::new(Mutex::new((Array::new(
        store,
        &format!("{GROUP_PATH}/{}", selection.level),
    ))?));
    let selection_arc = Arc::new(selection.clone());
    let mut tasks: Vec<JoinHandle<Result<()>>> = vec![];

    for y in selection.start.y..selection.end.y {
        for x in selection.start.x..selection.end.x {
            let array = Arc::clone(&array);
            let selection_arc = Arc::clone(&selection_arc);
            let sender = sender.clone();

            tasks.push(tokio::spawn(async move {
                #[cfg(feature = "time")]
                let start = Instant::now();

                // Retrieve chunk for each RGB channel.
                let channels = array.lock().unwrap().par_retrieve_chunks(
                    &ArraySubset::new_with_start_end_inc(
                        vec![0, 0, 0, y as u64, x as u64],
                        vec![0, 2, 0, y as u64, x as u64],
                    )?,
                )?;

                #[cfg(feature = "time")]
                println!(
                    "<{}:({}, {})>: Retrieving chunk took: {:?}",
                    selection_arc.level,
                    x,
                    y,
                    start.elapsed()
                );

                #[cfg(feature = "time")]
                let start = Instant::now();

                // TODO: Bottleneck #2.
                // Interleave RGB channels.
                let mut chunk = Vec::with_capacity(channels.len());
                for i in 0..CHUNK_LENGTH {
                    chunk.push(channels[i]);
                    chunk.push(channels[i + (CHUNK_LENGTH)]);
                    chunk.push(channels[i + (CHUNK_LENGTH * 2)]);
                }

                #[cfg(feature = "time")]
                println!(
                    "<{}:({}, {})>: Interleaving RGB channels took: {:?}",
                    selection_arc.level,
                    x,
                    y,
                    start.elapsed()
                );

                #[cfg(feature = "time")]
                let start = Instant::now();

                // TODO: Bottleneck #1.
                // Encode chunk to JPEG.
                let mut jpeg_chunk = Vec::new();
                JpegEncoder::new(&mut jpeg_chunk).encode(
                    &chunk,
                    CHUNK_SIZE,
                    CHUNK_SIZE,
                    image::ColorType::Rgb8,
                )?;

                #[cfg(feature = "time")]
                println!(
                    "<{}:({}, {})>: Encoding chunk to JPEG took: {:?}",
                    selection_arc.level,
                    x,
                    y,
                    start.elapsed()
                );

                #[cfg(feature = "time")]
                let start = Instant::now();

                // Prepend chunk position and level
                // (will be in this form [level, x, y, chunk...])
                jpeg_chunk.insert(0, y as u8);
                jpeg_chunk.insert(0, x as u8);
                jpeg_chunk.insert(0, selection_arc.level as u8);

                // Send chunk.
                let _ = sender
                    .send(Message::Binary(jpeg_chunk))
                    .await
                    .map_err(|err| {
                        eprintln!("Error sending chunk(s): {}", err);
                    });

                #[cfg(feature = "time")]
                println!(
                    "<{}:({}, {})>: Sending chunk took: {:?}\n",
                    selection_arc.level,
                    x,
                    y,
                    start.elapsed()
                );

                Ok(())
            }));
        }
    }

    // Await all tasks to complete
    join_all(tasks).await;

    Ok(())
}

// TODO: Generate using macros.
// TODO: Add extension checking function to Decoder to query decoders for supported extensions.
async fn open(image_path: &PathBuf) -> Result<impl Decoder> {
    if let Ok(image) = openslide_rs::OpenSlide::open(image_path) {
        return Ok(image);
    }

    Err(anyhow::anyhow!("Image could not be opened using any of the available decoders."))
}

// TODO: Don't cut out last row and column.
pub async fn convert(image_path: &PathBuf, store_path: &PathBuf) -> Result<Vec<Metadata>> {
    let image = open(image_path).await?;

    // One store per image.
    let store = Arc::new(FilesystemStore::new(store_path)?);

    // One group per image.
    let group = GroupBuilder::new().build(store.clone(), GROUP_PATH)?;

    // Write group metadata to store.
    // ! Remove group and make it so better adheres to OME-ZARR.
    group.store_metadata()?;

    let levels = image.get_level_count()?;
    if levels == 0 {
        return Err(anyhow::anyhow!("Image has no levels."));
    }

    println!("Levels: {}", levels);

    let mut metadata: Vec<Metadata> = Vec::new(); 

    for level in 0..levels {
        println!("Level: {}", level);
        // Get image dimensions.
        let (width, height) = image.get_level_dimensions(level)?;

        // Calculate number of chunks per row and column.
        let cols = width / CHUNK_SIZE;
        let rows = height / CHUNK_SIZE;

        // One array per image level.
        let array_path = format!("{}/{}", GROUP_PATH, level);

        let array = ArrayBuilder::new(
            // Define image shape.
            vec![0, RGB_CHANNELS, 0, height.into(), width.into()],
            // Define data type.
            DataType::UInt8,
            // Define chunk size.
            vec![1, 1, 1, CHUNK_SIZE.into(), CHUNK_SIZE.into()].try_into()?,
            // Define initial fill value.
            FillValue::from(0u8),
        )
        // Define compression algorithm and strength.
        .bytes_to_bytes_codecs(vec![
            #[cfg(feature = "lz4")]
            Box::new(codec::Lz4Codec::new(9)?),
        ])
        // Define dimension names - time, RGB channel, z, y, x axis.
        .dimension_names(vec!["t".into(), "c".into(), "z".into(), "y".into(), "x".into()].into())
        .build(store.clone(), &array_path)?;

        // Write array metadata to store.
        array.store_metadata()?;

        // Write chunk data.
        for y in 0..rows {
            for x in 0..cols {
                let chunk_grid: &Box<dyn ChunkGridTraits> = array.chunk_grid();

                // Read chunk region and split into separate RGB channels.
                let tile_split_channel: Vec<Vec<u8>> = image
                    .read_region(&Region {
                        size: Size {
                            width: CHUNK_SIZE,
                            height: CHUNK_SIZE,
                        },
                        level: level,
                        address: Address {
                            x: (x * CHUNK_SIZE),
                            y: (y * CHUNK_SIZE),
                        },
                    })
                    .unwrap()
                    .chunks(3)
                    .fold(
                        vec![Vec::new(), Vec::new(), Vec::new()],
                        |mut acc, chunk| {
                            acc[0].push(chunk[0]);
                            acc[1].push(chunk[1]);
                            acc[2].push(chunk[2]);
                            acc
                        },
                    );

                for c in 0..RGB_CHANNELS {
                    let chunk_indices: Vec<u64> = vec![0, c, 0, y.into(), x.into()];

                    if chunk_grid.subset(&chunk_indices, array.shape())?.is_some() {
                        let _ = array.store_chunk_elements(
                            &chunk_indices,
                            tile_split_channel[c as usize].clone(),
                        );
                    }
                }
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