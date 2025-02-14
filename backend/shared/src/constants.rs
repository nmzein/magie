pub static RGB_CHANNELS: u32 = 3;
pub static TILE_SIZE: u32 = 1024;
pub static TILE_LENGTH: usize = (TILE_SIZE * TILE_SIZE) as usize;
pub static TILE_SPLIT_LENGTH: usize = (TILE_SIZE * TILE_SIZE * RGB_CHANNELS) as usize;

pub static UPLOADED_DIRECTORY: &str = "uploaded";
pub static ANNOTATIONS_DIRECTORY: &str = "annotations";
pub static UPLOADED_IMAGE_PATH: &str = "uploaded/image";
pub static UPLOADED_ANNOTATIONS_PATH: &str = "uploaded/annotations";
pub static TRANSLATED_ANNOTATIONS_PATH: &str = "uploaded/annotations.json";
pub static IMAGE_PATH: &str = "image.zarr";
pub static THUMBNAIL_PATH: &str = "thumbnail.jpeg";
pub static ANNOTATIONS_PATH: &str = "annotations/a";

pub static PRIVILEDGED: [u32; 2] = [ROOT_ID, BIN_ID];
pub static ROOT_ID: u32 = 0;
pub static BIN_ID: u32 = 1;
