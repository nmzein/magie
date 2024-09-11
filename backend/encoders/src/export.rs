use shared::traits::Encoder;

pub fn get(name: &str) -> Option<Box<dyn Encoder>> {
    match name {
        crate::omezarr::NAME => Some(Box::new(crate::omezarr::Module)),
        _ => None,
    }
}

pub fn names() -> Vec<&'static str> {
    vec![
        crate::omezarr::NAME,
    ]
}
