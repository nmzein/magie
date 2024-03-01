use crate::generators::common::*;
use flate2::read::ZlibDecoder;
use geo_types::Geometry::Polygon;
use serde::Deserialize;
use sqlx::{sqlite::SqlitePool, FromRow};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::PathBuf,
};
use wkb::WKBReadExt;

#[derive(FromRow)]
struct Annotation {
    cx: u32,
    cy: u32,
    geometry: Vec<u8>,
    properties: String,
    area: f64,
}

#[derive(Deserialize)]
struct Properties {
    #[serde(rename = "type")]
    tag: String,
}

impl Annotation {
    fn parse_geometry(&self) -> Result<Vec<[f64; 2]>> {
        // Decompress zlib compressed geometry.
        let mut decoder = ZlibDecoder::new(&*self.geometry);
        let mut wkb = Vec::new();
        decoder.read_to_end(&mut wkb)?;

        // Read geometry stored in well-known bytes format.
        let mut cursor = Cursor::new(wkb);
        let Polygon(polygon) = cursor.read_wkb().unwrap() else {
            return Err(anyhow::anyhow!("Failed to read wkb."));
        };

        let (exterior, _) = polygon.into_inner();

        Ok(exterior.0.iter().map(|coord| [coord.x, coord.y]).collect())
    }

    fn parse_properties(&self) -> Result<Properties> {
        Ok(serde_json::from_str(&self.properties)?)
    }
}

const COLOURS_LEN: usize = 7;
static COLOURS: [&str; COLOURS_LEN] = [
    "#FF0000", // Red
    "#FF7F00", // Orange
    "#FFFF00", // Yellow
    "#00FF00", // Green
    "#0000FF", // Blue
    "#4B0082", // Indigo
    "#8B00FF", // Violet
];

// pub const name: &str = "TIAToolbox";

pub async fn read_annotations(annotations_path: &PathBuf) -> Result<Vec<AnnotationLayer>> {
    let database_url = format!("sqlite://{}", annotations_path.display());
    let pool = SqlitePool::connect(&database_url).await?;

    let start = std::time::Instant::now();

    let results = sqlx::query_as::<_, Annotation>(
        r#"
            SELECT cx, cy, geometry, properties, CAST(area AS REAL) as area
            FROM annotations
            ORDER BY area DESC
            LIMIT 200000;
        "#,
    )
    .fetch_all(&pool)
    .await?;

    println!("Query took: {:?}", start.elapsed());

    let start = std::time::Instant::now();

    let mut colour_index = 0;
    let mut layers: HashMap<String, AnnotationLayer> = HashMap::new();

    for result in results {
        let geometry = result.parse_geometry()?;
        let properties = result.parse_properties()?;
        let tag = properties.tag;

        let layer = layers.entry(tag.clone()).or_insert_with(|| {
            let fill = COLOURS[colour_index % COLOURS_LEN];
            colour_index += 1;
            println!("Creating layer for tag: {}", &tag);
            AnnotationLayer::new(tag, fill.into())
        });

        layer.insert(geometry);
    }

    println!("Decoding took: {:?}", start.elapsed());

    Ok(layers.into_values().collect())
}
