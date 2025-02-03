/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
        
pub fn get(extension: &str) -> Vec<impl Decoder> {
    match extension {
        "bif" => vec![
            crate::openslide::Module,
        ],
        "dcm" => vec![
            crate::openslide::Module,
        ],
        "mrxs" => vec![
            crate::openslide::Module,
        ],
        "ndpi" => vec![
            crate::openslide::Module,
        ],
        "scn" => vec![
            crate::openslide::Module,
        ],
        "svs" => vec![
            crate::openslide::Module,
        ],
        "svslide" => vec![
            crate::openslide::Module,
        ],
        "tif" => vec![
            crate::openslide::Module,
        ],
        "tiff" => vec![
            crate::openslide::Module,
        ],
        "vms" => vec![
            crate::openslide::Module,
        ],
        "vmu" => vec![
            crate::openslide::Module,
        ],
        _ => vec![],
    }
}

pub fn names(name: &str) -> Option<impl Decoder> {
    match name {
        crate::openslide::NAME => Some(crate::openslide::Module),
        _ => None,
    }
}
