#[cfg(feature = "zarr")]
pub trait Decoder: Sized {
    fn open(image_path: &std::path::PathBuf) -> anyhow::Result<Self>;
    fn get_level_count(&self) -> anyhow::Result<u32>;
    fn get_level_dimensions(&self, level: u32) -> anyhow::Result<(u32, u32)>;
    fn read_region(&self, region: &crate::structs::Region) -> anyhow::Result<image::RgbImage>;
}
