use anyhow::Result;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tempfile::NamedTempFile;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub current_image: Arc<Mutex<Option<ImageState>>>,
    pub generators: HashMap<String, Generator>,
}

#[derive(Clone)]
pub struct Generator {
    pub name: String,
    pub read_annotations: fn(annotations_path: &str) -> Result<Vec<AnnotationLayer>>,
}

#[derive(Clone, Debug)]
pub struct ImageState {
    pub image_path: PathBuf,
    pub store_path: PathBuf,
    pub annotations_path: Option<PathBuf>,
    pub metadata: Vec<Metadata>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Metadata {
    pub level: u32,
    pub cols: u32,
    pub rows: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileRequest {
    pub image_name: String,
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize)]
pub struct AnnotationLayer<'a> {
    pub tag: &'a str,
    pub colours: Colours<'a>,
    pub annotations: Vec<Vec<[u32; 2]>>,
}

#[derive(Debug, Serialize)]
pub struct Colours<'a> {
    pub fill: &'a str,
    pub stroke: &'a str,
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
