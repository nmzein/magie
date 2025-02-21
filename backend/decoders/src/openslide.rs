use crate::common::*;
use openslide_rs::{OpenSlide, traits::Slide};

pub struct Module {
    image: OpenSlide,
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

    fn open(image_path: &Path) -> Result<Self> {
        Ok(Self {
            image: OpenSlide::new(image_path)?,
        })
    }

    fn get_level_count(&self) -> Result<u32> {
        let levels = Slide::get_level_count(&self.image)?;

        Ok(levels)
    }

    fn get_level_dimensions(&self, level: u32) -> Result<(u32, u32)> {
        let image_dimensions = Slide::get_level_dimensions(&self.image, level)?;

        Ok((image_dimensions.w, image_dimensions.h))
    }

    fn read_region(&self, region: &Region) -> Result<Vec<u8>> {
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

        let tile = Slide::read_image_rgb(&self.image, &region)?.to_vec();

        Ok(tile)
    }

    fn thumbnail(&self, size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        use openslide_rs::Size;

        let thumbnail = self.image.thumbnail_rgb(&Size {
            w: size.width,
            h: size.height,
        })?;

        Ok(thumbnail)
    }
}
