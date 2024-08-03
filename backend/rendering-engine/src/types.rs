use anyhow::Result;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    // path::PathBuf,
    sync::{Arc, Mutex},
};
use tempfile::NamedTempFile;

pub type AppState = Arc<Mutex<Connection>>;

#[derive(Clone, Debug, Serialize)]
pub struct Directory {
    pub id: u32,
    pub name: String,
    #[serde(skip)]
    pub lft: u32,
    #[serde(skip)]
    pub rgt: u32,
    pub files: Vec<File>,
    pub subdirectories: Vec<Directory>,
}

#[derive(Clone, Debug, Serialize)]
pub struct File {
    pub id: u32,
    pub name: String,
    #[serde(skip)]
    pub parent_id: u32,
}

pub enum MoveMode {
    Regular,
    SoftDelete,
}

// #[derive(Clone, Debug)]
// pub struct ImageState {
//     pub directory_path: PathBuf,
//     pub image_name: String,
//     pub store_name: String,
//     pub annotations_name: Option<String>,
//     pub metadata_layers: Vec<MetadataLayer>,
// }

// #[derive(Debug)]
// pub struct Paths {
//     pub directory_path: PathBuf,
//     pub image_name: String,
//     pub store_name: String,
//     pub annotations_name: Option<String>,
// }

#[derive(Clone, Debug, Serialize)]
pub struct MetadataLayer {
    pub level: u32,
    pub cols: u32,
    pub rows: u32,
    pub width: u32,
    pub height: u32,
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
    pub parent_directory_id: u32,
    #[form_data(limit = "unlimited")]
    pub image_file: FieldData<NamedTempFile>,
    #[form_data(limit = "unlimited")]
    pub annotations_file: Option<FieldData<NamedTempFile>>,
    pub generator_name: String,
}
