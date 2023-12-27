use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;

pub type AppState = SqlitePool;

#[derive(Clone, Debug)]
pub struct ImageState {
    pub image_path: PathBuf,
    pub store_path: PathBuf,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Deserialize)]
pub struct ImageSelection {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub cols: u32,
    pub rows: u32,
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
