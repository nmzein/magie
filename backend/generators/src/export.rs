/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn get(name: &str) -> Option<impl Generator> {
    match name {
        crate::tiatoolbox::NAME => Some(crate::tiatoolbox::Module),
        _ => None,
    }
}

pub fn names() -> Vec<&'static str> {
    vec![
        crate::tiatoolbox::NAME,
    ]
}
