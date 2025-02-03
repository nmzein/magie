use crate::common::*;
use openslide_rs::{traits::Slide, OpenSlide};

pub const NAME: &str = "OpenSlide";
pub const EXTENSIONS: [&str; 11] = [
    "svs", // Aperio
    "tif", // Aperio, Trestle, Ventana, Generic tiled Tiff
    "dcm", // DICOM
    "vms", "vmu", "ndpi",    // Hamamatsu
    "scn",     // Leica
    "mrxs",    // MIRAX
    "tiff",    // Philips
    "svslide", // Sakura
    "bif",     // Ventana
];

pub struct Module;

impl Decoder for Module {
    fn name(&self) -> &'static str {
        NAME
    }

    fn extensions(&self) -> Vec<&'static str> {
        EXTENSIONS.into()
    }

    fn get_level_count(&self, image_path: &Path) -> Result<u32> {
        let image = OpenSlide::new(image_path)?;
        let levels = Slide::get_level_count(&image)?;

        Ok(levels)
    }

    fn get_level_dimensions(&self, image_path: &Path, level: u32) -> Result<(u32, u32)> {
        let image = OpenSlide::new(image_path)?;
        let image_dimensions = Slide::get_level_dimensions(&image, level)?;

        Ok((image_dimensions.w, image_dimensions.h))
    }

    fn read_region(&self, image_path: &Path, region: &Region) -> Result<Vec<u8>> {
        use openslide_rs::{Address, Region, Size};

        let image = OpenSlide::new(image_path)?;

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

    fn thumbnail(&self, image_path: &Path, size: &Size) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        use openslide_rs::Size;

        let image = OpenSlide::new(image_path)?;
        let thumbnail = image.thumbnail_rgb(&Size {
            w: size.width,
            h: size.height,
        })?;

        Ok(thumbnail)
    }
}
