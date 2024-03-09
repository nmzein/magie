use crate::common::*;
use openslide_rs::{traits::Slide, OpenSlide as ExternalOpenSlide};

pub struct OpenSlide;

impl Decoder for OpenSlide {
    fn name(&self) -> String {
        "OpenSlide".into()
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        [
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
        .into()
    }

    fn get_level_count(&self, image_path: &PathBuf) -> Result<u32> {
        let image = ExternalOpenSlide::new(image_path)?;
        let levels = Slide::get_level_count(&image)?;

        Ok(levels)
    }

    fn get_level_dimensions(&self, image_path: &PathBuf, level: u32) -> Result<(u32, u32)> {
        let image = ExternalOpenSlide::new(image_path)?;
        let image_dimensions = Slide::get_level_dimensions(&image, level)?;

        Ok((image_dimensions.w, image_dimensions.h))
    }

    fn read_region(&self, image_path: &PathBuf, region: &Region) -> Result<Vec<u8>> {
        use openslide_rs::{Address, Region, Size};

        let image = ExternalOpenSlide::new(image_path)?;

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

        let tile = Slide::read_image_rgb(&image, &region)?.to_vec();

        Ok(tile)
    }
}
