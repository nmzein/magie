use crate::generators::common::*;

use std::collections::HashMap;
use sqlx::{FromRow, sqlite::SqlitePool};

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
        "#
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
    
    Ok(vec![
        AnnotationLayer {
            tag: "Example 1",
            colours: Colours {
                fill: "#e0747099",
                stroke: "#a12c28",
            },
            annotations: annotations[0..10].to_vec(),
        },
    ])

    // let mut i = 0;
    // for val in nodes_result {
    //     println!("{}: {:?}", val.number, val.coords);
    //     i += 1;
    //     if i > 10 {
    //         break;
    //     }
    // }

    // Ok(vec![
    //     AnnotationLayer {
    //         tag: "Example 1",
    //         colours: Colours {
    //             fill: "#e0747099",
    //             stroke: "#a12c28",
    //         },
    //         annotations: vec![
    //             vec![
    //                 [20, 5000],
    //                 [220, 8000],
    //                 [230, 16000],
    //                 [30, 17000],
    //             ],
    //             vec![
    //                 [230, 200],
    //                 [500, 200],
    //                 [510, 300],
    //                 [310, 300],
    //             ],
    //         ],
    //     },
    //     AnnotationLayer {
    //         tag: "Example 2",
    //         colours: Colours {
    //             fill: "#719de399",
    //             stroke: "#2961ba",
    //         },
    //         annotations: vec![
    //             vec![
    //                 [230, 400],
    //                 [500, 400],
    //                 [510, 500],
    //                 [310, 500],
    //             ]
    //         ]
    //     },
    //     AnnotationLayer {
    //         tag: "Example 3",
    //         colours: Colours {
    //             fill: "#e0747099",
    //             stroke: "#a12c28",

    //         },
    //         annotations: vec![
    //             vec![
    //                 [1800, 1800],
    //                 [2048, 1800],
    //                 [2048, 2048],
    //                 [1800, 2048],
    //             ]
    //         ]
    //     },
    // ])
}
