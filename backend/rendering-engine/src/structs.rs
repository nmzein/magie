use anyhow::Result;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use shared::traits::{Decoder, Generator};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tempfile::NamedTempFile;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub current_image: Arc<Mutex<Option<ImageState>>>,
    pub decoders: Arc<Mutex<Vec<Box<dyn Decoder>>>>,
    pub generators: Arc<Mutex<HashMap<String, Box<dyn Generator>>>>,
}

#[derive(Clone, Debug)]
pub struct ImageState {
    pub directory_path: PathBuf,
    pub image_name: String,
    pub store_name: String,
    pub annotations_name: Option<String>,
    pub metadata: Vec<Metadata>,
}

#[derive(Debug)]
pub struct Paths {
    pub directory_path: PathBuf,
    pub image_name: String,
    pub store_name: String,
    pub annotations_name: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Metadata {
    pub level: u32,
    pub cols: u32,
    pub rows: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
pub struct ImageDataResponse {
    pub id: u32,
    pub path: String,
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
    pub directory_path: String,
    #[form_data(limit = "unlimited")]
    pub image: FieldData<NamedTempFile>,
    #[form_data(limit = "unlimited")]
    pub annotations: Option<FieldData<NamedTempFile>>,
    pub annotation_generator: String,
}
