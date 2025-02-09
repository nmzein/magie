use crate::common::*;

pub struct Module;

impl Generator for Module {
    fn name(&self) -> &'static str {
        "Test"
    }

    fn translate(&self, _annotations_path: &Path) -> Result<Vec<AnnotationLayer>> {
        Ok(Vec::new())
    }
}
