use crate::common::*;
use flate2::read::ZlibDecoder;
use geo_traits::to_geo::ToGeoGeometry;
use geo_types::Geometry;
use rusqlite::Connection;
use serde::Deserialize;
use std::{collections::HashMap, io::Read};
use wkb::reader;

pub const NAME: &str = "TIAToolbox";

pub struct Module;

impl Generator for Module {
    fn translate(&self, annotations_path: &Path) -> Result<Vec<AnnotationLayer>> {
        let start = std::time::Instant::now();

        let conn = Connection::open(annotations_path)?;

        let mut stmt = conn.prepare(
            r#"
                SELECT cx, cy, geometry, properties, CAST(area AS REAL) AS area
                FROM annotations
                ORDER BY area DESC;
            "#,
        )?;

        let annotations = stmt.query_map([], |row| {
            Ok(Annotation {
                _cx: row.get(0)?,
                _cy: row.get(1)?,
                geometry: row.get(2)?,
                properties: row.get(3)?,
                _area: row.get(4)?,
            })
        })?;

        // TODO: Move colour logic inside of AnnotationLayer w/ option for user to provide colours.
        let mut colour_index = 0;
        let mut layers = HashMap::new();

        for annotation in annotations {
            let annotation = annotation?;
            let geometry = annotation.parse_geometry()?;
            let properties = annotation.parse_properties()?;
            let tag = properties.tag;

            let layer = layers.entry(tag.clone()).or_insert_with(|| {
                let fill = COLOURS[colour_index % COLOURS_LEN];
                colour_index += 1;
                println!("Creating layer for tag: {}", &tag);
                AnnotationLayer::new(tag, fill.into())
            });

            layer.insert(geometry);
        }

        println!("Annotations took: {:?}", start.elapsed());

        Ok(layers.into_values().collect())
    }
}

struct Annotation {
    _cx: u32,
    _cy: u32,
    geometry: Vec<u8>,
    properties: String,
    _area: f64,
}

impl Annotation {
    fn parse_geometry(&self) -> Result<Vec<[f64; 2]>> {
        // Decompress zlib compressed geometry.
        let mut decoder = ZlibDecoder::new(&*self.geometry);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf)?;

        // Read geometry stored in well-known bytes format.
        let Ok(Some(polygon)) = reader::read_wkb(&mut buf).and_then(|g| Ok(g.try_to_geometry()))
        else {
            return Err(anyhow::anyhow!("Failed to read wkb."));
        };

        match polygon {
            Geometry::Polygon(polygon) => {
                let (exterior, _) = polygon.into_inner();

                Ok(exterior.0.iter().map(|coord| [coord.x, coord.y]).collect())
            }
            _ => Err(anyhow::anyhow!("Failed to retrieve polygon.")),
        }
    }

    fn parse_properties(&self) -> Result<Properties> {
        Ok(serde_json::from_str(&self.properties)?)
    }
}

#[derive(Deserialize)]
struct Properties {
    #[serde(rename = "type")]
    tag: String,
}

const COLOURS_LEN: usize = 8;
static COLOURS: [&str; COLOURS_LEN] = [
    "#FF0000", // Red
    "#FF7F00", // Orange
    "#FFFF00", // Yellow
    "#0000FF", // Blue
    "#FF1493", // DeepPink
    "#4B0082", // Indigo
    "#8B00FF", // Violet
    "#00FF00", // Green
];
