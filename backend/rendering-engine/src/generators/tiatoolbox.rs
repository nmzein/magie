use crate::generators::common::*;
use flate2::read::ZlibDecoder;
use geo_types::Geometry::Polygon;
use sqlx::{sqlite::SqlitePool, FromRow};
use std::{
    io::{Cursor, Read},
    path::PathBuf,
};
use wkb::WKBReadExt;

#[derive(Debug, Clone, FromRow)]
struct Annotation {
    cx: u32,
    cy: u32,
    geometry: Vec<u8>,
}

// pub const name: &str = "TIAToolbox"

pub async fn read_annotations(annotations_path: PathBuf) -> Result<Vec<AnnotationLayer<'static>>> {
    let database_url = format!("sqlite://{}", annotations_path.display());
    let pool = SqlitePool::connect(&database_url).await?;

    let start = std::time::Instant::now();

    let results = sqlx::query_as::<_, Annotation>(
        r#"
            SELECT cx, cy, geometry
            FROM annotations;
        "#,
    )
    .fetch_all(&pool)
    .await?;

    println!("Query took: {:?}", start.elapsed());

    let start = std::time::Instant::now();

    let mut annotations = Vec::with_capacity(results.len());
    for result in results {
        let mut decoder = ZlibDecoder::new(&*result.geometry);
        let mut wkb = Vec::new();
        decoder.read_to_end(&mut wkb)?;

        let mut cursor = Cursor::new(wkb);
        let Polygon(polygon) = cursor.read_wkb().unwrap() else {
            return Err(anyhow::anyhow!("Failed to read wkb."));
        };

        let (exterior, _) = polygon.into_inner();

        let annotation: Vec<_> = exterior
            .0
            .iter()
            .map(|coord| [coord.x as f32, coord.y as f32])
            .collect();

        annotations.push(annotation);
    }

    println!("Decoding took: {:?}", start.elapsed());

    let annotations = annotations[0..100000].to_vec();

    Ok(vec![AnnotationLayer {
        tag: "Connective Cell",
        visible: true,
        opacity: 1.0,
        fill: "#00d2ff",
        stroke: "#000000",
        annotations,
    }])
}
