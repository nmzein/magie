#[cfg(test)]
mod tests;

use crate::io;

use std::path::PathBuf;
use anyhow::Result;

use openslide_rs::{Address, Region, Size};
use openslide_rs::traits::Slide;

use image::{RgbImage, codecs::jpeg::JpegEncoder};

#[cfg(feature = "batch_write_to_hdf5")]
use hdf5::types::VarLenArray;

#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use hdf5::types::VarLenArray;
#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use std::thread;

// ! Temporary
static LEVEL: u32 = 0;
static TILE_SIZE_U32: u32 = 1024;
static TILE_SIZE_USIZE: usize = 1024;

#[cfg(feature = "immediately_write_to_hdf5")]
pub fn process(image_path: &PathBuf, hdf5_path: &PathBuf) -> Result<()> {
    let image = io::open_image(image_path)?;

    // Get image dimensions.
    let image_dimensions = image.get_level_dimensions(LEVEL)?;

    // Calculate number of tiles.
    let num_tiles = (image_dimensions.w as usize / TILE_SIZE_USIZE) * (image_dimensions.h as usize / TILE_SIZE_USIZE);

    // Create a new HDF5 file or fail if it already exists.
    let dataset = io::create_hdf5(hdf5_path, num_tiles)?;

    let mut current_tile: usize = 0;
    
    // Start reading tiles one row at a time from left to right.
    for y_offset in (0..image_dimensions.h).step_by(TILE_SIZE_USIZE) {
        for x_offset in (0..image_dimensions.w).step_by(TILE_SIZE_USIZE) {
            let tile = image.read_image_rgb(&Region {
                size: Size {
                    w: TILE_SIZE_U32,
                    h: TILE_SIZE_U32,
                },
                level: LEVEL,
                address: Address {
                    x: x_offset,
                    y: y_offset,
                },
            })?;

            let jpeg_tile = encode_tile_to_jpeg(&tile)?;

            let _ = io::write_to_hdf5(&dataset, &jpeg_tile, current_tile);

            println!("Tile {}", current_tile);
            current_tile += 1;
        }
    }

    Ok(())
}

#[cfg(feature = "batch_write_to_hdf5")]
pub fn process(image_path: &PathBuf, hdf5_path: &PathBuf) -> Result<()> {
    let image = io::open_image(image_path)?;

    // Get image dimensions.
    let image_dimensions = image.get_level_dimensions(LEVEL)?;

    // Vector for storing all the JPEG tiles in memory for all-at-once writing to HDF5.
    let mut jpeg_tiles: Vec<[VarLenArray<u8>; 1]> = Vec::new(); // vec![0; num_tiles]
    
    let mut current_tile: usize = 0;

    // Start reading tiles one row at a time from left to right.
    for y_offset in (0..image_dimensions.h).step_by(TILE_SIZE_USIZE) {
        for x_offset in (0..image_dimensions.w).step_by(TILE_SIZE_USIZE) {
            let tile = image.read_image_rgb(&Region {
                size: Size {
                    w: TILE_SIZE_U32,
                    h: TILE_SIZE_U32,
                },
                level: LEVEL,
                address: Address {
                    x: x_offset,
                    y: y_offset,
                },
            })?;

            let jpeg_tile = encode_tile_to_jpeg(&tile)?;
            jpeg_tiles.push([VarLenArray::from_slice(&jpeg_tile)]);

            println!("Tile {}", current_tile);
            current_tile += 1;
        }
    }

    let _ = io::write_to_hdf5(hdf5_path, &jpeg_tiles);

    Ok(())
}


#[cfg(feature = "parallelised_batch_write_to_hdf5")]
pub fn process(image_path: &PathBuf, hdf5_path: &PathBuf) -> Result<()> {
    use std::thread::JoinHandle;
    use std::sync::Arc;

    let image = io::open_image(image_path)?;

    // Get image dimensions.
    let image_dimensions = image.get_level_dimensions(LEVEL)?;

    // Vector for storing all the JPEG tiles in memory for all-at-once writing to HDF5.
    let mut jpeg_tiles: Vec<[VarLenArray<u8>; 1]> = Vec::new();
    
    let image = Arc::new(image);
    // Create a vector to hold thread handles
    let mut handles: Vec<JoinHandle<anyhow::Result<Vec<Vec<u8>>>>> = vec![];
    // let mut handles = vec![];

    for y_offset in (0..image_dimensions.h).step_by(TILE_SIZE_USIZE) {
        let image_clone = Arc::clone(&image);
        let handle = thread::spawn(move || {
            let mut row_jpeg_tiles = Vec::new();

            for x_offset in (0..image_dimensions.w).step_by(TILE_SIZE_USIZE) {
                let tile = image_clone.read_image_rgb(&Region {
                    size: Size {
                        w: TILE_SIZE_U32,
                        h: TILE_SIZE_U32,
                    },
                    level: LEVEL,
                    address: Address {
                        x: x_offset,
                        y: y_offset,
                    },
                })?;
    
                let jpeg_tile = encode_tile_to_jpeg(&tile)?;
                row_jpeg_tiles.push(jpeg_tile);
            }
            
            Ok(row_jpeg_tiles)
        });

        handles.push(handle);
    }

    for handle in handles {
        let row_jpeg_tiles = handle.join().expect("Thread panicked")?;
        let _ = row_jpeg_tiles.iter().for_each(|jpeg_tile| jpeg_tiles.push([VarLenArray::from_slice(&jpeg_tile)]));
    }

    let _ = io::write_to_hdf5(hdf5_path, &jpeg_tiles);

    Ok(())
}

fn encode_tile_to_jpeg(tile: &RgbImage) -> Result<Vec<u8>> {
    // Create an in-memory writer to hold the JPEG data.
    let mut jpeg_tile: Vec<u8> = Vec::new();
    
    // Encode the tile as JPEG.
    let mut encoder = JpegEncoder::new(&mut jpeg_tile);
    encoder.encode(tile, TILE_SIZE_U32, TILE_SIZE_U32, image::ColorType::Rgb8)?;

    Ok(jpeg_tile)
}