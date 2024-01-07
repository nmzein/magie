use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub plugins: HashMap<String, Plugin>,
}

#[derive(Clone)]
pub struct Plugin {
    pub name: fn() -> String,
}

#[derive(Debug)]
pub struct ImageState {
    pub image_path: PathBuf,
    pub store_path: PathBuf,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Selection {
    pub image_name: String,
    pub level: u32,
    pub start: Point,
    pub end: Point,
}
#[derive(Debug, Serialize)]
pub struct AnnotationLayer {
    pub tag: String,
    pub colours: Colours,
    pub annotations: Vec<Vec<Point>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub cols: u32,
    pub rows: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Serialize)]
pub struct Colours {
    pub fill: String,
    pub stroke: String,
}

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

#[derive(TryFromMultipart)]
pub struct UploadAssetRequest {
    #[form_data(limit = "unlimited")]
    pub image: FieldData<NamedTempFile>,
    #[form_data(limit = "unlimited")]
    pub annotations: Option<FieldData<NamedTempFile>>,
    pub annotation_generator: String,
}
