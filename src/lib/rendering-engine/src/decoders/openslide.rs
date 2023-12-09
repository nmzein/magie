use crate::structs::Region;
use crate::traits::Decoder;
use anyhow::Result;
use image::RgbImage;
use openslide_rs::{traits::Slide, OpenSlide};
use std::path::PathBuf;

#[cfg(feature = "zarr")]
impl Decoder for OpenSlide {
    fn open(image_path: &PathBuf) -> Result<OpenSlide> {
        let image = OpenSlide::new(image_path)?;

        Ok(image)
    }

    fn get_level_count(&self) -> Result<u32> {
        let levels = Slide::get_level_count(self)?;

        Ok(levels)
    }

    fn get_level_dimensions(&self, level: u32) -> Result<(u32, u32)> {
        let image_dimensions = Slide::get_level_dimensions(self, level)?;

        Ok((image_dimensions.h, image_dimensions.w))
    }

    fn read_region(&self, region: &Region) -> Result<RgbImage> {
        use openslide_rs::{Address, Region, Size};

        let region = Region {
            size: Size {
                w: region.size.width,
                h: region.size.height,
            },
            level: region.level,
            address: Address {
                x: region.address.x,
                y: region.address.y,
            },
        };

        let tile = Slide::read_image_rgb(self, &region)?;

        Ok(tile)
    }
}
