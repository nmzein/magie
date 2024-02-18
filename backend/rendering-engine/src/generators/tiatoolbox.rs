use crate::generators::common::*;
use sqlx::{sqlite::SqlitePool, FromRow};
use std::collections::HashMap;

#[derive(Debug, Clone, FromRow)]
struct Node {
    number: u32,
    x: u32,
    y: u32,
}

type Annotation = Vec<[u32; 2]>;

// pub const name: &str = "TIAToolbox";

pub async fn read_annotations(annotations_path: &str) -> Result<Vec<AnnotationLayer>> {
    let database_url = format!("sqlite://{}", annotations_path);
    let pool = SqlitePool::connect(&database_url).await?;

    let result = sqlx::query_as::<_, Node>(
        r#"
        SELECT nodeno AS number, cx AS x, cy AS y
        FROM annotations
        JOIN rtree_rowid ON annotations.id = rtree_rowid.rowid
        ORDER BY number;
        "#,
    )
    .fetch_all(&pool)
    .await?;

    // Aggregate coordinates by node number
    let mut nodes_map: HashMap<u32, Annotation> = HashMap::new();
    for node in result {
        let coords = nodes_map.entry(node.number).or_insert_with(Vec::new);
        coords.push([node.x, node.y]);
    }

    // Create a vector of annotations.
    let annotations: Vec<_> = nodes_map.values().cloned().collect();

    Ok(vec![AnnotationLayer {
        tag: "Example 1",
        colours: Colours {
            fill: "#e0747099",
            stroke: "#a12c28",
        },
        annotations: annotations[0..10].to_vec(),
    }])
}
