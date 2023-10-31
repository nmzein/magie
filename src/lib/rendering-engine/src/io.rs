#[cfg(test)]
mod tests;

use std::path::PathBuf;
use anyhow::Result;
use openslide_rs::OpenSlide;
use hdf5::File;
use ndarray::s;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;

#[cfg(feature = "write_to_fs")]
use image::RgbImage;

#[cfg(feature = "immediately_write_to_hdf5")]
use ndarray::ArrayView;
#[cfg(feature = "immediately_write_to_hdf5")]
use hdf5::{
    Dataset,
    SimpleExtents,
};

#[cfg(feature = "batch_write_to_hdf5")]
use ndarray::{Array, arr2, Ix1};
#[cfg(feature = "batch_write_to_hdf5")]
use hdf5::types::{VarLenArray};

#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use ndarray::{Array, arr2, Ix1};
#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use hdf5::types::{VarLenArray};


pub fn open_image(image_path: &PathBuf) -> Result<OpenSlide> {
    let image = OpenSlide::new(image_path)?;

    Ok(image)
}

#[cfg(feature = "write_to_fs")]
pub fn write_to_fs_png(tile: &RgbImage, current_tile: usize) -> Result<()> {
    use std::fs::File;
    use std::io::BufWriter;
    use image::ImageOutputFormat::Png;

    let output_path = PathBuf::from(format!("test_temp/tile_{}.png", current_tile));
    
    // Open a file for writing.
    let output_file = File::create(output_path)?;
    
    // Create a BufWriter for writing the PNG image.
    let mut writer = BufWriter::new(output_file);

    // Write the PNG data to the file.
    tile.write_to(&mut writer, Png)?;

    Ok(())
}

#[cfg(feature = "write_to_fs")]
pub fn write_to_fs_jpeg(tile: &[u8], current_tile: usize) -> Result<()> {
    use std::fs::File;
    use std::io::Write;

    let output_path = PathBuf::from(format!("test_temp/tile_{}.png", current_tile));

    // Open a file for writing.
    let mut file = File::create(output_path)?;

    // Write the JPEG data to the file.
    file.write_all(&tile)?;

    Ok(())
}

#[cfg(feature = "immediately_write_to_hdf5")]
pub fn create_hdf5(hdf5_path: &PathBuf, num_tiles: usize) -> Result<Dataset> {
    // Create a new HDF5 file or fail if it already exists.
    File::create_excl(hdf5_path)?;

    // Open HDF5 file for reading and writing.
    let hdf5_file = File::open_rw(hdf5_path)?;

    // Define the dataset dimensions.
    // Bad, adds 0 padding to end of tile.
    let max_tile_length: usize = 110000;
    let dataset_dimensions = SimpleExtents::new([num_tiles, max_tile_length]);

    // Set number of blosc threads.
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2); 
    
    let builder = hdf5_file.new_dataset::<u8>();
    
    // Define compression type and level (lz4, compression level 9, w/ shuffle).
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_lz4(9, true);
    
    let dataset = builder
        .shape(dataset_dimensions)
        .create("tiles")?;

    Ok(dataset)
}

#[cfg(feature = "immediately_write_to_hdf5")]
pub fn write_to_hdf5(dataset: &Dataset, tile: &[u8], current_tile: usize) -> Result<()> {
    let tile_length = tile.len();

    // Transform the tile data into a 2D array where the first value is its depth and the second its length.
    let tile = ArrayView::from_shape((1, tile_length), tile)?;
    
    // Define the slice of the dataset to write to. First value is the index and also encodes depth of 1.
    let slice = s![current_tile..current_tile+1, 0..tile_length];
    
    // Write the tile data to the dataset.
    dataset.write_slice(&tile, slice)?;
    
    Ok(())
}

#[cfg(feature = "immediately_write_to_hdf5")]
pub fn read_hdf5(hdf5_path: &PathBuf) -> Result<()> {
    // Open HDF5 file for reading.
    let hdf5_file = File::open(hdf5_path)?;

    // Open the dataset.
    let dataset = hdf5_file.dataset("tiles")?;
    
    // Read the entire dataset.
    let data = dataset.read_2d::<u8>()?;

    // println!("Data: {:?}", data);

    Ok(())
}

#[cfg(feature = "batch_write_to_hdf5")]
pub fn write_to_hdf5(hdf5_path: &PathBuf, tiles: &[[VarLenArray<u8>; 1]]) -> Result<()> {
    // Create a new HDF5 file or fail if it already exists.
    File::create_excl(hdf5_path)?;

    // Open HDF5 file for reading and writing.
    let hdf5_file = File::open_rw(hdf5_path)?;

    // Set number of blosc threads.
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2); 

    let builder = hdf5_file.new_dataset_builder();

    // Define compression type and level (lz4, compression level 9, w/ shuffle).
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_lz4(9, true);

    let _ = builder
        .with_data(&arr2(tiles))
        .create("tiles")?;

    Ok(())
}

#[cfg(feature = "batch_write_to_hdf5")]
pub fn read_hdf5(hdf5_path: &PathBuf) -> Result<()> {
    // Open HDF5 file for reading.
    let file = File::open(hdf5_path)?;

    // Open the dataset.
    let dataset = file.dataset("tiles")?;

    // Read the entire dataset.
    let data = dataset.read_2d::<u8>()?;

    // println!("Data: {:?}", data);

    // Read the second 1D array.
    // let data: Array<VarLenArray<u8>, Ix1> = dataset.read_slice_1d(s![1..2, 0])?;

    // println!("Second Array Data: {:?}", data[0]);

    Ok(())
}


#[cfg(feature = "parallelised_batch_write_to_hdf5")]
pub fn write_to_hdf5(hdf5_path: &PathBuf, tiles: &[[VarLenArray<u8>; 1]]) -> Result<()> {
    // Create a new HDF5 file or fail if it already exists.
    File::create_excl(hdf5_path)?;

    // Open HDF5 file for reading and writing.
    let hdf5_file = File::open_rw(hdf5_path)?;

    // Set number of blosc threads.
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2); 

    let builder = hdf5_file.new_dataset_builder();

    // Define compression type and level (lz4, compression level 9, w/ shuffle).
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_lz4(9, true);

    let _ = builder
        .with_data(&arr2(tiles))
        .create("tiles")?;

    Ok(())
}

#[cfg(feature = "parallelised_batch_write_to_hdf5")]
pub fn read_hdf5(hdf5_path: &PathBuf) -> Result<()> {
    // Open HDF5 file for reading.
    let file = File::open(hdf5_path)?;

    // Open the dataset.
    let dataset = file.dataset("tiles")?;

    // Read the entire dataset.
    let data = dataset.read_2d::<u8>()?;

    // println!("Data: {:?}", data);

    // Read the second 1D array.
    // let data: Array<VarLenArray<u8>, Ix1> = dataset.read_slice_1d(s![1..2, 0])?;

    // println!("Second Array Data: {:?}", data[0]);

    Ok(())
}