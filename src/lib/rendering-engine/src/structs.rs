use hdf5::H5Type;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::mem;
use std::path::PathBuf;

pub type State = BTreeMap<String, ImageState>;

#[derive(Clone, Debug)]
pub struct ImageState {
    pub image_path: PathBuf,
    pub datastore_path: PathBuf,
    pub image_metadata: ImageMetadata,
}

#[derive(Debug, Deserialize)]
pub struct ImageSelection {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, H5Type)]
#[repr(C)]
pub struct ImageMetadata {
    pub cols: usize,
    pub rows: usize,
}

#[cfg(feature = "batch_write_to_hdf5")]
impl AsRef<[u8]> for ImageMetadata {
    fn as_ref(&self) -> &[u8] {
        // Serialize the struct to a vector of bytes
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<Self>()) }
    }
}
