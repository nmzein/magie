#[cfg(test)]
mod tests;

use crate::structs::{ImageMetadata, ImageSelection};

use anyhow::Result;
use hdf5::File;
use ndarray::s;

use std::path::PathBuf;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;

#[cfg(feature = "write_to_fs")]
use image::RgbImage;

#[cfg(feature = "immediately_write_to_hdf5")]
use hdf5::{Dataset, SimpleExtents};
#[cfg(feature = "immediately_write_to_hdf5")]
use ndarray::ArrayView;

#[cfg(feature = "batch_write_to_hdf5")]
use hdf5::types::VarLenArray;
#[cfg(feature = "batch_write_to_hdf5")]
use ndarray::{arr2, Array, Ix1};

#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use hdf5::types::VarLenArray;
#[cfg(feature = "parallelised_batch_write_to_hdf5")]
use ndarray::{arr2, Array, Ix1};

#[cfg(feature = "write_to_fs")]
pub fn write_to_fs_png(tile: &RgbImage, current_tile: usize) -> Result<()> {
    use image::ImageOutputFormat::Png;
    use std::fs::File;
    use std::io::BufWriter;

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

    let dataset = builder.shape(dataset_dimensions).create("tiles")?;

    Ok(dataset)
}

#[cfg(feature = "immediately_write_to_hdf5")]
pub fn write_to_hdf5(dataset: &Dataset, tile: &[u8], current_tile: usize) -> Result<()> {
    let tile_length = tile.len();

    // Transform the tile data into a 2D array where the first value is its depth and the second its length.
    let tile = ArrayView::from_shape((1, tile_length), tile)?;

    // Define the slice of the dataset to write to. First value is the index and also encodes depth of 1.
    let slice = s![current_tile..current_tile + 1, 0..tile_length];

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

    let _ = builder.with_data(&arr2(tiles)).create("tiles")?;
    Ok(())
}

#[cfg(feature = "batch_write_to_hdf5")]
pub fn read_hdf5(
    hdf5_path: &PathBuf,
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
    let file = File::open(hdf5_path)?;

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

    let _ = builder.with_data(&arr2(tiles)).create("tiles")?;

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

    Ok(())
}

#[cfg(feature = "zarr")]
use image::{ImageBuffer, Rgb};
#[cfg(feature = "zarr")]
use openslide_rs::{Address, OpenSlide, Region, Size};
#[cfg(feature = "zarr")]
use std::sync::Arc;
#[cfg(feature = "zarr")]
use zarrs::{
    array::ArrayBuilder,
    array::FillValue,
    array::{chunk_grid::ChunkGridTraits, DataType},
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

#[cfg(feature = "zarr")]
pub trait Image {
    fn open(image_path: &PathBuf) -> Result<Self>
    where
        Self: Sized;
    fn levels(&self) -> Result<u32>;
    fn level_dimensions(&self, level: u32) -> Result<(u64, u64)>;
    fn read_region(&self, region: &Region) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>;
}

#[cfg(feature = "zarr")]
impl Image for OpenSlide {
    fn open(image_path: &PathBuf) -> Result<OpenSlide> {
        let image = OpenSlide::new(image_path)?;

        Ok(image)
    }

    fn levels(&self) -> Result<u32> {
        use openslide_rs::traits::Slide;
        let levels = self.get_level_count()?;

        Ok(levels)
    }

    fn level_dimensions(&self, level: u32) -> Result<(u64, u64)> {
        use openslide_rs::traits::Slide;
        let image_dimensions = self.get_level_dimensions(level)?;

        Ok((image_dimensions.h.into(), image_dimensions.w.into()))
    }

    fn read_region(&self, region: &Region) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        use openslide_rs::traits::Slide;
        let tile = self.read_image_rgb(region)?;

        Ok(tile)
    }
}

#[cfg(feature = "zarr")]
pub fn convert_to_zarr<T: Image>(image_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    const CHUNK_SIZE: u64 = 64;
    const PIXELS: usize = 64 * 64;
    const RGB_CHANNELS: u64 = 3;

    let image: T = Image::open(image_path)?;

    // One store per image.
    let store = Arc::new(FilesystemStore::new(output_path)?);

    // One group per image.
    let group_path = "/group";
    let group = GroupBuilder::new().build(store.clone(), group_path)?;

    // Write group metadata to store.
    group.store_metadata()?;

    let levels = image.levels()?;

    for level in 0..levels {
        // Get image dimensions.
        let (height, width) = image.level_dimensions(level)?;

        // Calculate number of chunks per row and column.
        let rows = height / CHUNK_SIZE;
        let cols = width / CHUNK_SIZE;

        // One array per image level.
        let array_path = format!("{}/{}", group_path, level);

        let array = ArrayBuilder::new(
            // Define image shape.
            vec![0, RGB_CHANNELS, 0, height, width],
            // Define data type.
            DataType::UInt8,
            // Define chunk size.
            vec![1, 1, 1, CHUNK_SIZE, CHUNK_SIZE].into(),
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
                            w: CHUNK_SIZE as u32,
                            h: CHUNK_SIZE as u32,
                        },
                        level: level,
                        address: Address {
                            x: (x * CHUNK_SIZE) as u32,
                            y: (y * CHUNK_SIZE) as u32,
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
                    let chunk_indices: Vec<u64> = vec![0, c, 0, y, x];

                    if chunk_grid.subset(&chunk_indices, array.shape())?.is_some() {
                        let _ = array.store_chunk_elements(
                            &chunk_indices,
                            tile_split_channel[c as usize].clone(),
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
