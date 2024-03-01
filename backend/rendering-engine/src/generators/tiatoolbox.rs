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
    type_: String,
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

#[derive(FromRow)]
struct Extremes {
    min: f64,
    max: f64,
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

const DESIRED_RESOLUTION_LAYERS: u32 = 6;

pub async fn read_annotations(annotations_path: &PathBuf) -> Result<Vec<AnnotationLayer>> {
    let database_url = format!("sqlite://{}", annotations_path.display());
    let pool = SqlitePool::connect(&database_url).await?;

    let start = std::time::Instant::now();

    // let extremes = sqlx::query_as::<_, Extremes>(
    //     r#"
    //         SELECT MIN(area) AS min, MAX(area) AS max
    //         FROM annotations;
    //     "#,
    // )
    // .fetch_one(&pool)
    // .await?;

    // Find range between max and min, divide into N slices.
    // let slice = (extremes.max - extremes.min) / (DESIRED_RESOLUTION_LAYERS as f64);
    // // Take middle N slices as thresholds ignoring first and last.
    // // Start with highest area and decrement by slice.
    // let mut area_thresholds = vec![];
    // for t in 0..DESIRED_RESOLUTION_LAYERS - 1 {
    //     area_thresholds.push(extremes.max - (t as f64) * slice);
    // }

    // let area_thresholds = vec![15000.0, 10000.0, 500.0, 450.0, 300.0];

    // println!("Thresholds: {:?}", area_thresholds);

    let results = sqlx::query_as::<_, Annotation>(
        r#"
            SELECT cx, cy, geometry, properties, CAST(area AS REAL) as area
            FROM annotations
            ORDER BY area DESC
            LIMIT 50000;
        "#,
    )
    .fetch_all(&pool)
    .await?;

    println!("Query took: {:?}", start.elapsed());

    let start = std::time::Instant::now();

    let mut colour_index = 0;
    let mut layers: HashMap<String, AnnotationLayer> = HashMap::new();
    let mut current_depth: usize = 0;

    for result in results {
        let geometry = result.parse_geometry()?;
        let properties = result.parse_properties()?;

        // If not in last layer depth i.e. if desired is 5 and
        // we are in 3 (4th layer), dont look ahead just continue till end
        // ! Set to use area_thresh instead
        // if current_depth != (DESIRED_RESOLUTION_LAYERS - 2) as usize
        //     && result.area < area_thresholds[current_depth + 1]
        // {
        //     println!(
        //         "Result area: {}, next threshold area: {}, current_depth: {}",
        //         result.area,
        //         area_thresholds[current_depth + 1],
        //         current_depth
        //     );
        //     current_depth += 1;
        // }

        let tag = properties.type_;
        if let Some(layer) = layers.get_mut(&tag) {
            layer.insert(geometry, current_depth);
        } else {
            let fill = COLOURS[colour_index % COLOURS_LEN];
            colour_index += 1;

            println!("Creating layer for tag: {}", &tag);

            let mut layer =
                AnnotationLayer::new(tag.clone(), fill.into(), DESIRED_RESOLUTION_LAYERS as usize);
            layer.insert(geometry, current_depth);

            layers.insert(tag, layer);
        }
    }

    println!("Decoding took: {:?}", start.elapsed());

    Ok(layers.values().cloned().collect())
}
