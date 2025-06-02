use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Clone, Debug, Serialize)]
pub struct AnnotationLayers {
    count: usize,
    layers: HashMap<String, AnnotationLayer>,
    colours: Vec<String>,
}

impl Default for AnnotationLayers {
    fn default() -> Self {
        Self {
            count: 0,
            layers: HashMap::new(),
            colours: vec![
                "#FF0000".into(), // Red
                "#FF7F00".into(), // Orange
                "#FFFF00".into(), // Yellow
                "#0000FF".into(), // Blue
                "#FF1493".into(), // DeepPink
                "#4B0082".into(), // Indigo
                "#8B00FF".into(), // Violet
                "#00FF00".into(), // Green
            ],
        }
    }
}

impl AnnotationLayers {
    pub fn new(colours: Vec<String>) -> Self {
        Self {
            count: 0,
            layers: HashMap::new(),
            colours,
        }
    }

    pub fn insert(&mut self, tag: String, geometry: Vec<[f64; 2]>) {
        let layer = self.layers.entry(tag.clone()).or_insert_with(|| {
            let fill = &self.colours[self.count % self.colours.len()];
            let new_layer = AnnotationLayer::new(self.count, tag, fill.into());
            self.count += 1;
            new_layer
        });

        layer.insert(geometry);
    }

    pub fn to_vec(self) -> Vec<AnnotationLayer> {
        self.layers.into_values().collect()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AnnotationLayer {
    pub id: usize,
    pub tag: String,
    pub visible: bool,
    pub opacity: f32,
    pub fill: String,
    pub stroke: String,
    pub annotations: Vec<Vec<[f64; 2]>>,
}

impl AnnotationLayer {
    pub fn new(id: usize, tag: String, fill: String) -> Self {
        Self {
            id,
            tag,
            visible: true,
            opacity: 0.5,
            fill,
            stroke: "#000000".into(),
            annotations: vec![],
        }
    }

    pub fn insert(&mut self, geometry: Vec<[f64; 2]>) {
        self.annotations.push(geometry);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MetadataLayer {
    pub level: u32,
    pub cols: u32,
    pub rows: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum FileSystemEntry {
    Directory(Directory),
    Asset(Asset),
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}

pub enum MoveMode {
    Regular,
    SoftDelete,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum DeleteMode {
    #[serde(alias = "hard")]
    Hard,
    #[serde(alias = "soft")]
    Soft,
}

#[derive(Clone, Debug, Serialize)]
pub struct ImageProperties {
    pub metadata: Vec<MetadataLayer>,
    pub annotations: Vec<AnnotationLayer>,
}
