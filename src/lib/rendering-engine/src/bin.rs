mod image;
mod io;

use std::{fs, path::PathBuf, time::Instant};
use anyhow::Result;

fn main() -> Result<()>{
    let image_path = PathBuf::from("images/image.tiff");
    let hdf5_path = PathBuf::from("test_temp/test_image_process.h5");

    // Cleanup from previous test.
    let _ = fs::remove_file(&hdf5_path);

    let now = Instant::now();

    image::process(&image_path, &hdf5_path)?;
    
    let elapsed = now.elapsed();
    println!("Time to process: {:.2?}", elapsed);

    io::read_hdf5(&hdf5_path)?;

    let elapsed = now.elapsed();
    println!("Time to read: {:.2?}", elapsed);

    // Cleanup.
    // let _ = fs::remove_file(&hdf5_path);

    Ok(())
}