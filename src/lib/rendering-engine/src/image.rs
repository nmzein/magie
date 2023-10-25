use std::path::PathBuf;
use openslide_rs::OpenSlide;
use openslide_rs::traits::Slide;

// TODO: Replace with application logic
pub fn decode(file_path: PathBuf) -> anyhow::Result<(u32, u32)> {
    // Open file
    let file = OpenSlide::new(&file_path)?;
    
    // Get width and height of image at given level
    let dimensions = file.get_level_dimensions(0)?;

    Ok((dimensions.w, dimensions.h))
}