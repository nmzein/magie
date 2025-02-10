use crate::common::*;

pub struct Module;

impl Decoder for Module {
    fn name(&self) -> &'static str {
        "Test"
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["tiff"]
    }

    fn open(_image_path: &Path) -> Result<Self> {
        Ok(Self {})
    }

    fn get_level_count(&self) -> Result<u32> {
        Ok(0)
    }

    fn get_level_dimensions(&self, _level: u32) -> Result<(u32, u32)> {
        Ok((0, 0))
    }

    fn read_region(&self, _region: &Region) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn thumbnail(&self, _size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        Ok(ImageBuffer::new(0, 0))
    }
}
