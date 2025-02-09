/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
pub fn get(name: &str) -> Option<Box<dyn Encoder>> {
    match name {
        "OMEZarr" => Some(Box::new(crate::omezarr::Module)),
        "Test" => Some(Box::new(crate::test::Module)),
        _ => None,
    }
}
pub fn names() -> Vec<&'static str> {
    vec!["OMEZarr", "Test"]
}
