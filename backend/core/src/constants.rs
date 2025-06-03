pub static LOCAL_STORES_PATH: &str = env!("LOCAL_STORES_PATH");
pub static LOCAL_DATABASES_PATH: &str = env!("LOCAL_DATABASES_PATH");
pub static REGISTRY_PATH: &str = concat!(env!("LOCAL_DATABASES_PATH"), "registry.sqlite");
pub static REGISTRY_URL: &str = concat!("sqlite://", "../_databases/", "registry.sqlite");

pub static UPLOADED_DIRECTORY: &str = "uploaded";
pub static ANNOTATIONS_DIRECTORY: &str = "annotations";
pub static UPLOADED_IMAGE_PATH: &str = "uploaded/image";
pub static UPLOADED_ANNOTATIONS_PATH: &str = "uploaded/annotations";
pub static TRANSLATED_ANNOTATIONS_PATH: &str = "uploaded/annotations.json";
pub static IMAGE_NAME: &str = "image.zarr";
pub static THUMBNAIL_NAME: &str = "thumbnail.jpeg";
pub static ANNOTATIONS_PATH_PREFIX: &str = "annotations/a";

pub static MAX_THUMBNAIL_SIZE: u32 = 256;

pub static PRIVILEGED: [u32; 2] = [ROOT_ID, BIN_ID];
pub static ROOT_ID: u32 = 0;
pub static BIN_ID: u32 = 1;
