use crate::structs::{AnnotationLayer, MetadataLayer, Region, Size};
use anyhow::Result;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub trait Decoder: Send + Sync {
    fn name(&self) -> &'static str;
    fn extensions(&self) -> Vec<&'static str>;
    fn open(&mut self, image_path: &Path) -> Result<()>;
    fn get_level_count(&self) -> Result<u32>;
    fn get_level_dimensions(&self, level: u32) -> Result<(u32, u32)>;
    fn read_region(&self, region: &Region) -> Result<Vec<u8>>;
    fn thumbnail(&self, size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>;
}

pub trait Encoder: Send + Sync {
    fn name(&self) -> &'static str;
    fn convert(&self, output_path: &Path, decoder: Box<dyn Decoder>) -> Result<Vec<MetadataLayer>>;
    fn retrieve(&self, image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>>;
}

pub trait Generator: Send + Sync {
    fn name(&self) -> &'static str;
    fn translate(&self, annotations_path: &Path) -> Result<Vec<AnnotationLayer>>;
}
