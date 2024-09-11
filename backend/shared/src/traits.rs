use crate::structs::{AnnotationLayer, MetadataLayer, Region};
use anyhow::Result;
use std::path::Path;

pub trait Decoder: Send {
    fn get_level_count(&self, image_path: &Path) -> Result<u32>;
    fn get_level_dimensions(&self, image_path: &Path, level: u32) -> Result<(u32, u32)>;
    fn read_region(&self, image_path: &Path, region: &Region) -> Result<Vec<u8>>;
}

pub trait Encoder: Send {
    fn convert(
        input_path: &Path,
        output_path: &Path,
        decoder: Box<dyn Decoder>,
    ) -> Result<Vec<MetadataLayer>>;
    fn retrieve(image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>>;
}

pub trait Generator: Send + Sync {
    fn translate(&self, annotations_path: &Path) -> Result<Vec<AnnotationLayer>>;
}
