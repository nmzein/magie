#[cfg(test)]
mod tests;

use crate::structs::{Address, ImageMetadata, ImageSelection, Region, Size};
use anyhow::Result;
use std::path::PathBuf;

// use crate::image::encode_tile_to_jpeg;
// use hdf5::File;
// use ndarray::s;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
#[cfg(feature = "hdf5")]
use hdf5::types::VarLenArray;
#[cfg(feature = "hdf5")]
use ndarray::{arr2, Array, Ix1};

#[cfg(feature = "zarr")]
use crate::traits::Decoder;
#[cfg(feature = "zarr")]
use image::codecs::jpeg::JpegEncoder;
#[cfg(feature = "zarr")]
use itertools::izip;
// #[cfg(feature = "zarr")]
// use ndarray::{s, ArrayD};
#[cfg(feature = "zarr")]
use std::sync::Arc;
#[cfg(feature = "zarr")]
use zarrs::{
    array::{chunk_grid::ChunkGridTraits, Array, ArrayBuilder, DataType, FillValue},
    // array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

#[cfg(feature = "hdf5")]
pub fn write_metadata(metadata_path: &PathBuf, image_metadata: ImageMetadata) -> Result<()> {
    use std::fs::File;
    use std::io::Write;

    let metadata_json = serde_json::to_string(&image_metadata)?;

    // Open a file for writing.
    let mut file = File::create(metadata_path)?;

    // Write the JSON data to the file.
    file.write_all(metadata_json.as_bytes())?;

    Ok(())
}

#[cfg(feature = "hdf5")]
pub fn read_metadata(metadata_path: &PathBuf) -> Result<ImageMetadata> {
    use std::fs::File;
    use std::io::Read;

    // Open a file for reading.
    let mut file = File::open(metadata_path)?;

    // Read the JSON data from the file.
    let mut metadata_json = String::new();
    file.read_to_string(&mut metadata_json)?;

    // Parse the JSON data.
    let image_metadata: ImageMetadata = serde_json::from_str(&metadata_json)?;

    Ok(image_metadata)
}

#[cfg(feature = "hdf5")]
pub fn retrieve(
    store_path: &PathBuf,
    selection: &ImageSelection,
    image_metadata: &ImageMetadata,
) -> Result<Vec<Vec<u8>>> {
    // Perform some basic checks on the selection.
    // ! Implement client side checks too.
    if selection.start.x > selection.end.x
        || selection.start.y > selection.end.y
        || selection.end.x > image_metadata.cols - 1
        || selection.end.y > image_metadata.rows - 1
    {
        return Err(anyhow::anyhow!("Invalid selection."));
    }

    // Open HDF5 file for reading.
    let file = File::open(store_path)?;

    // Open the dataset.
    let dataset = file.dataset("tiles")?;

    let mut result = Vec::new();

    // Get selection.
    for current_row in selection.start.y..=selection.end.y {
        let row_offset = current_row * image_metadata.cols;

        let start_pos = row_offset + selection.start.x;
        let end_pos = row_offset + selection.end.x + 1;

        let data: Array<VarLenArray<u8>, Ix1> = dataset.read_slice_1d(s![start_pos..end_pos, 0])?;

        result.extend(data.into_iter().map(|d| d.to_vec()));
    }

    Ok(result)
}

#[cfg(feature = "hdf5")]
pub fn convert(store_path: &PathBuf, tiles: &[[VarLenArray<u8>; 1]]) -> Result<()> {
    // Create a new HDF5 file or fail if it already exists.
    File::create_excl(store_path)?;

    // Open HDF5 file for reading and writing.
    let hdf5_file = File::open_rw(store_path)?;

    // Set number of blosc threads.
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2);

    let builder = hdf5_file.new_dataset_builder();

    // Define compression type and level (lz4, compression level 9, w/ shuffle).
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_lz4(9, true);

    let _ = builder.with_data(&arr2(tiles)).create("tiles")?;
    Ok(())
}

// #[cfg(feature = "zarr")]
// pub fn retrieve(
//     store_path: &PathBuf,
//     level: &u64,
//     selection: &ImageSelection,
// ) -> Result<Vec<Vec<u8>>> {
//     const CHUNK_SIZE: u32 = 64;
//     const RGB_CHANNELS: u32 = 3;
//     const GROUP_PATH: &str = "/group";
//     const RED: usize = 0;
//     const GREEN: usize = 1;
//     const BLUE: usize = 2;

//     let store = Arc::new(FilesystemStore::new(store_path)?);
//     let array = Array::new(store.clone(), &format!("{}/{}", GROUP_PATH, level))?;

//     let subset = ArraySubset::new_with_start_end_inc(
//         vec![0, 0, 0, selection.start.y as u64, selection.start.x as u64],
//         vec![0, 2, 0, selection.end.y as u64, selection.end.x as u64],
//     )?;

//     let mut tiles: ArrayD<u8> = array.retrieve_array_subset_ndarray(&subset)?;
//     let mut combined_tiles: Vec<u8> = Vec::new();

//     for y in 0..tiles.shape()[3] {
//         for x in 0..tiles.shape()[4] {
//             println!("Y: {}, X: {}", y, x);
//             let red = tiles.slice(s![0, RED..GREEN, 0, y..y + 1, x..x + 1]);
//             let green = tiles.slice(s![0, GREEN..BLUE, 0, y..y + 1, x..x + 1]);
//             let blue = tiles.slice(s![0, BLUE.., 0, y..y + 1, x..x + 1]);

//             println!("Red: {:?}", red);
//             println!("Green: {:?}", green);
//             println!("Blue: {:?}", blue);

//             // Interleave RGB channels.
//             let tile: Vec<u8> = izip!(red, green, blue)
//                 .flat_map(|(r, g, b)| vec![*r, *g, *b])
//                 .collect();

//             println!("Tile: {:?}", tile);

//             let mut jpeg_tile = Vec::new();
//             JpegEncoder::new(&mut jpeg_tile)
//                 .encode(&tile, CHUNK_SIZE, CHUNK_SIZE, image::ColorType::Rgb8)
//                 .unwrap();

//             combined_tiles.extend(jpeg_tile);
//         }
//     }

//     // let mut tiles = tiles
//     //     .map(|channels: ArrayD<u8>| -> Vec<u8> {
//     //         let mut tile = Vec::new();
//     //         for (r, g, b) in izip!(
//     //             channels.slice(s![0, 0, 0, .., ..]).iter(),
//     //             channels.slice(s![0, 1, 0, .., ..]).iter(),
//     //             channels.slice(s![0, 2, 0, .., ..]).iter()
//     //         ) {
//     //             tile.push(*r);
//     //             tile.push(*g);
//     //             tile.push(*b);
//     //         }

//     //         let mut jpeg_tile = Vec::new();
//     //         JpegEncoder::new(&mut jpeg_tile)
//     //             .encode(&tile, CHUNK_SIZE, CHUNK_SIZE, image::ColorType::Rgb8)
//     //             .unwrap();

//     //         jpeg_tile
//     //     })
//     //     .collect();

//     // .map(|channels: &[Vec<u8>]| {
//     //     let mut tile = Vec::new();
//     //     for (r, g, b) in izip!(
//     //         channels[0].into_iter(),
//     //         channels[1].into_iter(),
//     //         channels[2].into_iter()
//     //     ) {
//     //         tile.push(r);
//     //         tile.push(g);
//     //         tile.push(b);
//     //     }

//     //     let mut jpeg_tile = Vec::new();
//     //     JpegEncoder::new(&mut jpeg_tile)
//     //         .encode(&tile, CHUNK_SIZE, CHUNK_SIZE, image::ColorType::Rgb8)
//     //         .unwrap();

//     //     jpeg_tile
//     // })
//     // .collect();

//     // .flat_map(|chunk| chunk.to_vec())
//     // .collect();

//     println!("Tiles: {:?}", tiles.len());

//     Ok(Vec::new())
// }

static CHUNK_SIZE: u32 = 1024;
static RGB_CHANNELS: u64 = 3;
static GROUP_PATH: &str = "/group";

#[cfg(feature = "zarr")]
pub fn retrieve(
    store_path: &PathBuf,
    level: &u64,
    selection: &ImageSelection,
) -> Result<Vec<Vec<u8>>> {
    let store = Arc::new(FilesystemStore::new(store_path)?);
    let array = Array::new(store.clone(), &format!("{}/{}", GROUP_PATH, level))?;

    let mut tiles = Vec::new();

    for y in selection.start.y..selection.end.y {
        for x in selection.start.x..selection.end.x {
            // Retrieve chunk for each RGB channel.
            let channels: Vec<Vec<u8>> = (0..RGB_CHANNELS)
                .map(|c| {
                    array
                        .retrieve_chunk(&[0, c, 0, y as u64, x as u64])
                        .expect("Failed to retrieve chunk.")
                        .to_vec()
                })
                .collect();

            // Interleave RGB channels.
            let tile: Vec<u8> = izip!(
                channels[0].clone(),
                channels[1].clone(),
                channels[2].clone()
            )
            .flat_map(|(r, g, b)| vec![r, g, b])
            .collect();

            // Encode tile to JPEG.
            let mut jpeg_tile = Vec::new();

            JpegEncoder::new(&mut jpeg_tile).encode(
                &tile,
                CHUNK_SIZE,
                CHUNK_SIZE,
                image::ColorType::Rgb8,
            )?;

            // TODO: Send immediately instead of storing.
            tiles.push(jpeg_tile);
        }
    }

    Ok(tiles)
}

#[cfg(feature = "zarr")]
pub fn convert<T: Decoder>(image_path: &PathBuf, store_path: &PathBuf) -> Result<ImageMetadata> {
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
        let (height, width) = image.get_level_dimensions(level)?;

        // Calculate number of chunks per row and column.
        let rows = height / CHUNK_SIZE;
        let cols = width / CHUNK_SIZE;

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
        return Ok(ImageMetadata { cols, rows });
    }

    Err(anyhow::anyhow!("Image has no levels."))
}
