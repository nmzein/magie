#[cfg(test)]
mod tests;

// use anyhow::Result;
// use image::{codecs::jpeg::JpegEncoder, RgbImage};

// // use crate::io;
// // use crate::structs::ImageMetadata;

// // use std::path::PathBuf;

// // use openslide_rs::traits::Slide;
// // use openslide_rs::{Address, OpenSlide, Region, Size};

// #[cfg(feature = "hdf5")]
// use hdf5::types::VarLenArray;

// #[cfg(feature = "hdf5")]
// static LEVEL: u32 = 0;
// #[cfg(feature = "hdf5")]
// static TILE_SIZE_U32: u32 = 1024;
// #[cfg(feature = "hdf5")]
// static TILE_SIZE_USIZE: usize = 1024;

// #[cfg(feature = "hdf5")]
// pub fn process(image_path: &PathBuf, hdf5_path: &PathBuf) -> Result<ImageMetadata> {
//     let image = OpenSlide::new(image_path)?;

//     // Get image dimensions.
//     let image_dimensions = image.get_level_dimensions(LEVEL)?;

//     // Vector for storing all the JPEG tiles in memory for all-at-once writing to HDF5.
//     let mut jpeg_tiles: Vec<[VarLenArray<u8>; 1]> = Vec::new();

//     // Start reading tiles one row at a time from left to right.
//     for y_offset in (0..image_dimensions.h).step_by(TILE_SIZE_USIZE) {
//         for x_offset in (0..image_dimensions.w).step_by(TILE_SIZE_USIZE) {
//             let tile = image.read_image_rgb(&Region {
//                 size: Size {
//                     w: TILE_SIZE_U32,
//                     h: TILE_SIZE_U32,
//                 },
//                 level: LEVEL,
//                 address: Address {
//                     x: x_offset,
//                     y: y_offset,
//                 },
//             })?;

//             let jpeg_tile = encode_tile_to_jpeg(&tile)?;
//             jpeg_tiles.push([VarLenArray::from_slice(&jpeg_tile)]);
//         }
//     }

//     let image_metadata = ImageMetadata {
//         cols: image_dimensions.w as usize / TILE_SIZE_USIZE,
//         rows: image_dimensions.h as usize / TILE_SIZE_USIZE,
//     };

//     let _ = io::write_to_hdf5(hdf5_path, &jpeg_tiles);

//     Ok(image_metadata)
// }

// #[cfg(feature = "hdf5")]
// pub fn encode_tile_to_jpeg(tile: &RgbImage) -> Result<Vec<u8>> {
//     // Create an in-memory writer to hold the JPEG data.
//     let mut jpeg_tile: Vec<u8> = Vec::new();

//     // Encode the tile as JPEG.
//     let mut encoder = JpegEncoder::new(&mut jpeg_tile);
//     encoder.encode(tile, TILE_SIZE_U32, TILE_SIZE_U32, image::ColorType::Rgb8)?;

//     Ok(jpeg_tile)
// }
