/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
pub fn get(extension: &str, image_path: &Path) -> Option<Box<dyn Decoder>> {
    match extension {
        "bif" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "dcm" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "mrxs" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "ndpi" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "scn" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "svs" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "svslide" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "tif" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "tiff" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            if let Ok(decoder) = crate::test::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "vms" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        "vmu" => {
            if let Ok(decoder) = crate::openslide::Module::open(image_path) {
                return Some(Box::new(decoder));
            }
            None
        }
        _ => None,
    }
}
pub fn names() -> Vec<&'static str> {
    vec!["OpenSlide", "Test"]
}
