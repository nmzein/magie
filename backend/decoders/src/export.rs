/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
pub fn get(extension: &str) -> Vec<Box<dyn Decoder>> {
    match extension {
        "bif" => vec![Box::new(crate::openslide::Module::default())],
        "dcm" => vec![Box::new(crate::openslide::Module::default())],
        "mrxs" => vec![Box::new(crate::openslide::Module::default())],
        "ndpi" => vec![Box::new(crate::openslide::Module::default())],
        "scn" => vec![Box::new(crate::openslide::Module::default())],
        "svs" => vec![Box::new(crate::openslide::Module::default())],
        "svslide" => vec![Box::new(crate::openslide::Module::default())],
        "test" => vec![Box::new(crate::test::Module::default())],
        "tif" => vec![Box::new(crate::openslide::Module::default())],
        "tiff" => vec![Box::new(crate::openslide::Module::default())],
        "vms" => vec![Box::new(crate::openslide::Module::default())],
        "vmu" => vec![Box::new(crate::openslide::Module::default())],
        _ => vec![],
    }
}
pub fn names() -> Vec<&'static str> {
    vec!["OpenSlide", "Test"]
}
