use crate::structs::Region;
use anyhow::Result;
use image::RgbImage;
use std::path::PathBuf;

pub trait Decoder: Sized {
    fn open(image_path: &PathBuf) -> Result<Self>;
    fn get_level_count(&self) -> Result<u32>;
    fn get_level_dimensions(&self, level: u32) -> Result<(u32, u32)>;
    fn read_region(&self, region: &Region) -> Result<RgbImage>;
}
