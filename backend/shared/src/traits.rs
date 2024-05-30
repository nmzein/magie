use crate::structs::{AnnotationLayer, Region};
use anyhow::Result;
use std::path::PathBuf;

pub trait Decoder: Send {
    fn get_level_count(&self, image_path: &PathBuf) -> Result<u32>;
    fn get_level_dimensions(&self, image_path: &PathBuf, level: u32) -> Result<(u32, u32)>;
    fn read_region(&self, image_path: &PathBuf, region: &Region) -> Result<Vec<u8>>;
}

pub trait Generator: Send + Sync {
    fn translate(&self, annotations_path: &PathBuf) -> Result<Vec<AnnotationLayer>>;
}
