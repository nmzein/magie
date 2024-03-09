pub use anyhow::Result;
pub use shared::{structs::AnnotationLayer, traits::Generator};
use std::collections::HashMap;

// TODO: Avoid manual registration of generators.
#[no_mangle]
pub fn get_generators() -> HashMap<String, Box<dyn Generator>> {
    let mut generators = HashMap::new();
    let tiatoolbox = crate::tiatoolbox::TIAToolbox;

    generators.insert(
        tiatoolbox.name(),
        Box::new(tiatoolbox) as Box<dyn Generator>,
    );

    generators
}
