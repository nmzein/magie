use anyhow::Result;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use shared::structs::MetadataLayer;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;

pub type AppState = Arc<Mutex<Connection>>;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Directory {
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: u32,
    pub name: String,
    #[serde(skip)]
    pub lft: u32,
    #[serde(skip)]
    pub rgt: u32,
    pub files: Vec<File>,
    pub subdirectories: Vec<Directory>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct File {
    #[serde(rename = "type")]
    pub r#type: String,
    pub id: u32,
    pub name: String,
    #[serde(skip)]
    pub parent_id: u32,
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

#[derive(Clone, Debug, Serialize)]
pub struct AnnotationLayer {
    pub id: u32,
    pub tag: String,
    pub visible: bool,
    pub opacity: f32,
    pub fill: String,
    pub stroke: String,
}

impl AnnotationLayer {
    pub fn new(id: u32, tag: String) -> Self {
        Self {
            id,
            tag,
            visible: true,
            opacity: 0.5,
            fill: "#FF0000".to_string(),
            stroke: "#000000".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileRequest {
    pub id: u32,
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(TryFromMultipart)]
pub struct UploadAssetRequest {
    pub decoder: String,
    pub encoder: String,
    pub generator: Option<String>,
    #[form_data(limit = "unlimited")]
    pub image_file: FieldData<NamedTempFile>,
    #[form_data(limit = "unlimited")]
    pub annotations_file: Option<FieldData<NamedTempFile>>,
}
