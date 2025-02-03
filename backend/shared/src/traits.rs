use crate::structs::{AnnotationLayer, MetadataLayer, Region, Size};
use anyhow::Result;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub trait Decoder: Send + Sync {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> Vec<&'static str>;
    fn get_level_count(&self, image_path: &Path) -> Result<u32>;
    fn get_level_dimensions(&self, image_path: &Path, level: u32) -> Result<(u32, u32)>;
    fn read_region(&self, image_path: &Path, region: &Region) -> Result<Vec<u8>>;
    fn thumbnail(&self, image_path: &Path, size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>;
}

pub trait Encoder: Send + Sync {
    fn name(&self) -> &'static str;
    fn convert<D: Decoder>(
        &self,
        input_path: &Path,
        output_path: &Path,
        decoder: D,
    ) -> Result<Vec<MetadataLayer>>;
    fn retrieve(&self, image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>>;
}

pub trait Generator: Send + Sync {
    fn name(&self) -> &'static str;
    fn translate(&self, annotations_path: &Path) -> Result<Vec<AnnotationLayer>>;
}
