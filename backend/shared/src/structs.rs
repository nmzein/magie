use serde::Serialize;

pub struct Region {
    pub size: Size,
    pub level: u32,
    pub address: Address,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct Address {
    pub x: u32,
    pub y: u32,
}

pub type Geometry = Vec<[f64; 2]>;
#[derive(Clone, Debug, Serialize)]
pub struct AnnotationLayer {
    pub tag: String,
    pub visible: bool,
    pub opacity: f32,
    pub fill: String,
    pub stroke: String,
    pub annotations: Vec<Geometry>,
}

impl AnnotationLayer {
    pub fn new(tag: String, fill: String) -> Self {
        Self {
            tag,
            visible: true,
            opacity: 0.5,
            fill,
            stroke: "#000000".into(),
            annotations: vec![],
        }
    }

    pub fn insert(&mut self, geometry: Geometry) {
        self.annotations.push(geometry);
    }
}
