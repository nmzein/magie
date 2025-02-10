use crate::common::*;

pub struct Module;

impl Encoder for Module {
    fn name(&self) -> &'static str {
        "Test"
    }

    fn convert(
        &self,
        _output_path: &Path,
        _decoder: &Box<dyn Decoder>,
    ) -> Result<Vec<MetadataLayer>> {
        Ok(Vec::new())
    }

    fn retrieve(
        &self,
        _buf: &mut Box<[u8]>,
        _image_path: &Path,
        _level: u32,
        _x: u32,
        _y: u32,
    ) -> Result<()> {
        Ok(())
    }
}
