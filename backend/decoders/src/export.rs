use shared::traits::Decoder;
        
pub fn get() -> Vec<Box<dyn Decoder>> {
    vec![
        Box::new(crate::openslide::OpenSlide)
    ]
}
