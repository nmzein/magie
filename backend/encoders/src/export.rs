/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn get(name: &str) -> Option<impl Encoder> {
    match name {
        crate::omezarr::NAME => Some(crate::omezarr::Module),
        _ => None,
    }
}

pub fn names() -> Vec<&'static str> {
    vec![
        crate::omezarr::NAME,
    ]
}
