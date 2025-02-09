use crate::common::*;
use openslide_rs::{traits::Slide, OpenSlide};

pub struct Module {
    image: Option<OpenSlide>,
}

impl Default for Module {
    fn default() -> Self {
        Self { image: None }
    }
}

impl Decoder for Module {
    fn name(&self) -> &'static str {
        "OpenSlide"
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec![
            "svs", // Aperio
            "tif", // Aperio, Trestle, Ventana, Generic tiled Tiff
            "dcm", // DICOM
            "vms", "vmu", "ndpi",    // Hamamatsu
            "scn",     // Leica
            "mrxs",    // MIRAX
            "tiff",    // Philips
            "svslide", // Sakura
            "bif",     // Ventana
        ]
    }

    fn open(&mut self, image_path: &Path) -> Result<()> {
        self.image = Some(OpenSlide::new(image_path)?);

        Ok(())
    }

    fn get_level_count(&self) -> Result<u32> {
        if let Some(image) = &self.image {
            let levels = Slide::get_level_count(image)?;

            return Ok(levels);
        }

        Err(anyhow::anyhow!("Image not opened"))
    }

    fn get_level_dimensions(&self, level: u32) -> Result<(u32, u32)> {
        if let Some(image) = &self.image {
            let image_dimensions = Slide::get_level_dimensions(image, level)?;

            return Ok((image_dimensions.w, image_dimensions.h));
        }

        Err(anyhow::anyhow!("Image not opened"))
    }

    fn read_region(&self, region: &Region) -> Result<Vec<u8>> {
        if let Some(image) = &self.image {
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

            let tile = Slide::read_image_rgb(image, &region)?.to_vec();

            return Ok(tile);
        }

        Err(anyhow::anyhow!("Image not opened"))
    }

    fn thumbnail(&self, size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        if let Some(image) = &self.image {
            use openslide_rs::Size;

            let thumbnail = image.thumbnail_rgb(&Size {
                w: size.width,
                h: size.height,
            })?;

            return Ok(thumbnail);
        }

        Err(anyhow::anyhow!("Image not opened"))
    }
}
