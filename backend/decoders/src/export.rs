/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
        
pub fn get(extension: &str) -> Vec<Box<dyn Decoder>> {
    match extension {
        "bif" => vec![
            Box::new(crate::openslide::Module),
        ],
        "dcm" => vec![
            Box::new(crate::openslide::Module),
        ],
        "mrxs" => vec![
            Box::new(crate::openslide::Module),
        ],
        "ndpi" => vec![
            Box::new(crate::openslide::Module),
        ],
        "scn" => vec![
            Box::new(crate::openslide::Module),
        ],
        "svs" => vec![
            Box::new(crate::openslide::Module),
        ],
        "svslide" => vec![
            Box::new(crate::openslide::Module),
        ],
        "tif" => vec![
            Box::new(crate::openslide::Module),
        ],
        "tiff" => vec![
            Box::new(crate::openslide::Module),
        ],
        "vms" => vec![
            Box::new(crate::openslide::Module),
        ],
        "vmu" => vec![
            Box::new(crate::openslide::Module),
        ],
        _ => vec![],
    }
}
