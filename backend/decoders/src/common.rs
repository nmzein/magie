pub use anyhow::Result;
pub use shared::{structs::Region, traits::Decoder};
pub use std::path::PathBuf;

// TODO: Avoid manual registration of decoders.
#[no_mangle]
pub fn get_decoders() -> Vec<Box<dyn Decoder>> {
    vec![Box::new(crate::openslide::OpenSlide) as Box<dyn Decoder>]
}
