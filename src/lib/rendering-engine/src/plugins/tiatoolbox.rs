include!("traits.rs");

pub struct TIAToolbox;

impl AnnotationGenerator for TIAToolbox {
    #[no_mangle]
    fn name() -> String {
        "TIA Toolbox".to_string()
    }
}
