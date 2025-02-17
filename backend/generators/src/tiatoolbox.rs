use crate::common::*;
use flate2::read::ZlibDecoder;
use geo_traits::to_geo::ToGeoGeometry;
use geo_types::Geometry;
use rusqlite::{Connection, Error};
use serde::Deserialize;
use shared::types::AnnotationLayers;
use std::io::Read;
use wkb::reader;

pub struct Module;

impl Generator for Module {
    fn name(&self) -> &'static str {
        "TIAToolbox"
    }

    fn translate(&self, annotations_path: &Path) -> Result<Vec<AnnotationLayer>> {
        let mut timer = Timer::new("generators/tiatoolbox/translate");

        let conn = Connection::open(annotations_path)?;

        let mut stmt = conn.prepare(
            "
                SELECT cx, cy, geometry, properties, CAST(area AS REAL) AS area
                FROM annotations
                ORDER BY area DESC;
            ",
        )?;

        let annotations = stmt.query_map([], |row| {
            Ok(Annotation {
                _cx: row.get(0)?,
                _cy: row.get(1)?,
                geometry: parse_geometry(row.get(2)?)?,
                properties: parse_properties(row.get(3)?)?,
                _area: row.get(4)?,
            })
        })?;

        timer.lap("Fetched annotations from database.");

        let mut layers = AnnotationLayers::default();

        annotations.filter_map(Result::ok).for_each(|annotation| {
            layers.insert(annotation.properties.tag, annotation.geometry);
        });

        timer.end("Grouped annotations.");

        Ok(layers.to_vec())
    }
}

struct Annotation {
    _cx: u32,
    _cy: u32,
    geometry: Vec<[f64; 2]>,
    properties: Properties,
    _area: f64,
}

#[derive(Deserialize)]
struct Properties {
    #[serde(rename = "type")]
    tag: String,
}

fn parse_properties(properties: String) -> Result<Properties, rusqlite::Error> {
    match serde_json::from_str(&properties) {
        Ok(properties) => Ok(properties),
        Err(e) => Err(Error::ToSqlConversionFailure(e.into())),
    }
}

fn parse_geometry(geometry: Vec<u8>) -> Result<Vec<[f64; 2]>, rusqlite::Error> {
    // Decompress zlib compressed geometry.
    let mut decoder = ZlibDecoder::new(&*geometry);
    let mut buf = Vec::new();
    let Ok(_) = decoder.read_to_end(&mut buf) else {
        return Err(Error::ToSqlConversionFailure(
            "Failed to decode geometry.".into(),
        ));
    };

    // Read geometry stored in well-known bytes format.
    let Ok(Some(Geometry::Polygon(polygon))) = reader::read_wkb(&buf).map(|g| g.try_to_geometry())
    else {
        return Err(Error::ToSqlConversionFailure("Failed to read wkb.".into()));
    };

    let (exterior, _) = polygon.into_inner();

    Ok(exterior.0.iter().map(|coord| [coord.x, coord.y]).collect())
}
