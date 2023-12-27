// use crate::io;

// use anyhow::Result;
// use std::{fs, path::PathBuf};

// #[cfg(feature = "immediately_write_to_hdf5")]
// #[test]
// fn test_io_create_hdf5() -> Result<()> {
//     let num_tiles = 1500;
//     let tile_size: usize = 1024;
//     let max_tile_length: usize = 110_000;

//     let h5_path = PathBuf::from("test_temp/test_io_create_h5.h5");

//     // Cleanup from previous test in case it failed.
//     let _ = fs::remove_file(&h5_path);

//     // Get the dataset.
//     let dataset = io::create_hdf5(&h5_path, num_tiles)?;

//     // Verify that the dataset is 2D dataset.
//     let dataspace = dataset.space()?;
//     assert_eq!(dataspace.ndim(), 2);

//     // Get the dimensions.
//     let dimensions = dataspace.shape();

//     // Check if the dimensions match the expected values.
//     let expected_dimensions = [num_tiles, max_tile_length];
//     assert_eq!(dimensions, expected_dimensions);

//     // Cleanup.
//     let _ = fs::remove_file(&h5_path);

//     Ok(())
// }
