/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn convert(
    name: &str,
    input_path: &Path,
    output_path: &Path,
    decoder: Box<dyn Decoder>,
) -> Result<Vec<MetadataLayer>> {
    match name {
        crate::omezarr::NAME => crate::omezarr::Module::convert(input_path, output_path, decoder),
        _ => Err(anyhow::anyhow!("Encoder not found.")),
    }
}
pub fn retrieve(name: &str, image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>> {
    match name {
        crate::omezarr::NAME => crate::omezarr::Module::retrieve(image_path, level, x, y),
        _ => Err(anyhow::anyhow!("Encoder not found.")),
    }
}

pub fn names() -> Vec<&'static str> {
    vec![
        crate::omezarr::NAME,
    ]
}
