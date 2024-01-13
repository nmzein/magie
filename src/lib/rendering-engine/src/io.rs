use crate::structs::{Address, AnnotationLayer, Colours, Metadata, Point, Region, Selection, Size};
use crate::traits::Decoder;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use image::codecs::jpeg::JpegEncoder;
use tokio::fs;
use zarrs::{
    array::{chunk_grid::ChunkGridTraits, Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

static CHUNK_SIZE: u32 = 1024;
static CHUNK_LENGTH: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize;
static RGB_CHANNELS: u64 = 3;
static GROUP_PATH: &str = "/group";
static STORE_PATH: &str = "store";

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
pub async fn annotations(image_name: &str) -> Result<Vec<AnnotationLayer>> {
    let annotations = vec![
        AnnotationLayer {
            tag: "Example 1".into(),
            colours: Colours {
                fill: "#e0747099".into(),
                stroke: "#a12c28".into(),
            },
            annotations: [
                [
                    Point { x: 20, y: 50 },
                    Point { x: 220, y: 80 },
                    Point { x: 230, y: 160 },
                    Point { x: 30, y: 170 },
                ]
                .into(),
                [
                    Point { x: 230, y: 200 },
                    Point { x: 500, y: 200 },
                    Point { x: 510, y: 300 },
                    Point { x: 310, y: 300 },
                ]
                .into(),
            ]
            .into(),
        },
        AnnotationLayer {
            tag: "Example 2".into(),
            colours: Colours {
                fill: "#719de399".into(),
                stroke: "#2961ba".into(),
            },
            annotations: [[
                Point { x: 230, y: 400 },
                Point { x: 500, y: 400 },
                Point { x: 510, y: 500 },
                Point { x: 310, y: 500 },
            ]
            .into()]
            .into(),
        },
        AnnotationLayer {
            tag: "Example 3".into(),
            colours: Colours {
                fill: "#e0747099".into(),
                stroke: "#a12c28".into(),
            },
            annotations: [[
                Point { x: 1800, y: 1800 },
                Point { x: 2048, y: 1800 },
                Point { x: 2048, y: 2048 },
                Point { x: 1800, y: 2048 },
            ]
            .into()]
            .into(),
        },
    ];

    Ok(annotations)
}

pub async fn retrieve(
    store_path: &PathBuf,
    selection: &Selection,
    socket: &mut WebSocket,
) -> Result<()> {
    let store = Arc::new(FilesystemStore::new(store_path)?);
    let array = Array::new(
        store.clone(),
        &format!("{GROUP_PATH}/{}", selection.level),
    )?;

    for y in selection.start.y..selection.end.y {
        for x in selection.start.x..selection.end.x {
            // Retrieve chunk for each RGB channel.
            let channels = array.par_retrieve_chunks(
                &ArraySubset::new_with_start_end_inc(
                    vec![0, 0, 0, y as u64, x as u64],
                    vec![0, 2, 0, y as u64, x as u64],
                )?
            )?;

            // Interleave RGB channels.
            let mut tile = Vec::with_capacity(channels.len());
            for i in 0..CHUNK_LENGTH {
                tile.push(channels[i]);
                tile.push(channels[i + (CHUNK_LENGTH)]);
                tile.push(channels[i + (CHUNK_LENGTH * 2)]);
            }

            // Encode tile to JPEG.
            let mut jpeg_tile = Vec::new();
            JpegEncoder::new(&mut jpeg_tile).encode(
                &tile,
                CHUNK_SIZE,
                CHUNK_SIZE,
                image::ColorType::Rgb8,
            )?;

            // Prepend tile position.
            jpeg_tile.insert(0, y as u8);
            jpeg_tile.insert(0, x as u8);

            // Send tile.
            let _ = socket
                .send(Message::Binary(jpeg_tile))
                .await
                .map_err(|err| {
                    eprintln!("Error sending tile(s): {}", err);
                });
        }
    }

    Ok(())
}

// TODO: Don't cut out last row and column.
pub async fn convert<T: Decoder>(image_path: &PathBuf, store_path: &PathBuf) -> Result<Metadata> {
    let image: T = Decoder::open(image_path)?;

    // One store per image.
    let store = Arc::new(FilesystemStore::new(store_path)?);

    // One group per image.
    let group = GroupBuilder::new().build(store.clone(), GROUP_PATH)?;

    // Write group metadata to store.
    group.store_metadata()?;

    let levels = image.get_level_count()?;

    for level in 0..levels {
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
            vec![1, 1, 1, CHUNK_SIZE.into(), CHUNK_SIZE.into()].into(),
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

                // Read tile region and split into separate RGB channels.
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
        // Change to return vec of metadata.
        return Ok(Metadata {
            cols,
            rows,
            width,
            height,
        });
    }

    Err(anyhow::anyhow!("Image has no levels."))
}