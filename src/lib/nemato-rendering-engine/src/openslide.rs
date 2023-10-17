use std::path::PathBuf;
use openslide_rs::{OpenSlide, Size};
use openslide_rs::traits::Slide;

// TODO: Replace with application logic
pub fn decode(file_path: PathBuf) -> anyhow::Result<Size> {
    // Open file
    let file = OpenSlide::new(&file_path)?;
    
    // Get width and height of image at given level
    let dimensions = file.get_level_dimensions(1)?;

    Ok(dimensions)
}