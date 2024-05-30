use shared::traits::Generator;

pub fn get(name: &str) -> Option<Box<dyn Generator>> {
    match name {
        crate::tiatoolbox::NAME => Some(Box::new(crate::tiatoolbox::Module)),
        _ => None,
    }
}

pub fn names() -> Vec<&'static str> {
    vec![
        crate::tiatoolbox::NAME,
    ]
}
