use crate::image;

use anyhow::Result;
use std::{fs, path::PathBuf};

#[test]
fn test_image_process() -> Result<()> {
    let image_path = PathBuf::from("images/image.tiff");
    let h5_path = PathBuf::from("test_temp/test_image_process.h5");

    // Cleanup from previous test.
    let _ = fs::remove_file(&h5_path);

    image::process(&image_path, &h5_path)?;

    // Cleanup.
    let _ = fs::remove_file(&h5_path);

    Ok(())
}
