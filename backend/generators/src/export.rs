/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
pub fn get(name: &str) -> Option<Box<dyn Generator>> {
    match name {
        "Test" => Some(Box::new(crate::test::Module)),
        "TIAToolbox" => Some(Box::new(crate::tiatoolbox::Module)),
        _ => None,
    }
}
pub fn names() -> Vec<&'static str> {
    vec!["Test", "TIAToolbox"]
}
