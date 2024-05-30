use crate::types::{ImageDataResponse, ImageState, MetadataLayer, Paths};
use anyhow::Result;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use shared::structs::AnnotationLayer;
use std::{
    fmt::Debug,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub async fn connect(database_url: &str) -> Result<Connection> {
    let mut conn = Connection::open(database_url)?;

    let migrations = Migrations::new(vec![M::up(include_str!("../../state/schema.sql"))]);

    // Update the database schema atomically.
    migrations.to_latest(&mut conn)?;

    Ok(conn)
}

pub async fn list(conn: Arc<Mutex<Connection>>) -> Result<Vec<ImageDataResponse>> {
    let conn_lock = conn.lock().unwrap();
    let mut stmt = conn_lock.prepare(
        r#"
            SELECT id, directory_path FROM images;
        "#,
    )?;

    let list = stmt
        .query_map([], |row| {
            Ok(ImageDataResponse {
                id: row.get(0)?,
                path: row.get(1)?,
            })
        })?
        .map(|res| res.map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>, _>>();

    #[cfg(feature = "log.database")]
    log("LIST", Some(&list));

    list
}

pub async fn insert(
    directory_path: &PathBuf,
    image_name: &str,
    store_name: &str,
    annotations_name: Option<&str>,
    metadata_layers: Vec<MetadataLayer>,
    annotation_layers: Vec<AnnotationLayer>,
    conn: Arc<Mutex<Connection>>,
) -> Result<()> {
    let directory_path = directory_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Could not convert directory path to string."))?;

    let mut conn_lock = conn.lock().unwrap();
    let transaction = conn_lock.transaction()?;

    transaction.execute(
        r#"
            INSERT INTO images (directory_path, image_name, store_name, annotations_name)
            VALUES (?1, ?2, ?3, ?4);
        "#,
        (directory_path, image_name, store_name, annotations_name),
    )?;

    let image_id = transaction.last_insert_rowid();

    #[cfg(feature = "log.database")]
    log::<()>(&format!("INSERT <Image: {image_id}>"), None);

    for m in metadata_layers {
        transaction.execute(
            r#"
                INSERT INTO metadata_layer (image_id, level, cols, rows, width, height)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6);
            "#,
            (image_id, m.level, m.cols, m.rows, m.width, m.height),
        )?;

        #[cfg(feature = "log.database")]
        log::<()>(
            &format!("INSERT <Metadata Layer: {image_id}:{}>", m.level),
            None,
        );
    }

    for a in annotation_layers {
        transaction.execute(
            r#"
                INSERT INTO annotation_layer (image_id, tag)
                VALUES (?1, ?2);
            "#,
            (image_id, a.tag.clone()),
        )?;

        #[cfg(feature = "log.database")]
        log::<()>(
            &format!("INSERT <Annotation Layer: {image_id}:{}>", a.tag),
            None,
        );
    }

    let _ = transaction.commit();

    Ok(())
}

pub async fn contains(directory_path: &str, conn: Arc<Mutex<Connection>>) -> Result<bool> {
    let conn_lock: std::sync::MutexGuard<'_, Connection> = conn.lock().unwrap();
    let mut stmt = conn_lock.prepare(
        r#"
            SELECT 1 FROM images WHERE directory_path = ?1;
        "#,
    )?;

    let contains = stmt.exists(&[directory_path])?;

    #[cfg(feature = "log.database")]
    log(
        &format!("CONTAINS <Image: {}>", directory_path),
        Some(&contains),
    );

    Ok(contains)
}

pub async fn get_paths(id: u32, conn: Arc<Mutex<Connection>>) -> Result<Paths> {
    let conn_lock = conn.lock().unwrap();
    let mut stmt = conn_lock.prepare(
        r#"
            SELECT directory_path, image_name, store_name, annotations_name
            FROM images
            WHERE id = ?1;
        "#,
    )?;

    let paths = stmt.query_row([id], |row| {
        Ok(Paths {
            directory_path: PathBuf::from(row.get::<_, String>(0)?),
            image_name: row.get(1)?,
            store_name: row.get(2)?,
            annotations_name: Some(row.get(3)?),
        })
    })?;

    // #[cfg(feature = "log.database")]
    // log(&format!("GET <Paths: {}>", id), Some(&paths));

    Ok(paths)
}

pub async fn get_metadata_layers(
    id: u32,
    conn: Arc<Mutex<Connection>>,
) -> Result<Vec<MetadataLayer>> {
    let conn_lock = conn.lock().unwrap();
    let mut stmt = conn_lock.prepare(
        r#"
            SELECT level, cols, rows, width, height
            FROM metadata_layer
            WHERE image_id = ?1
            ORDER BY level ASC;
        "#,
    )?;

    let metadata_layer = stmt
        .query_map([id], |row| {
            Ok(MetadataLayer {
                level: row.get(0)?,
                cols: row.get(1)?,
                rows: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    #[cfg(feature = "log.database")]
    log(&format!("GET <Metadata: {}>", id), Some(&metadata_layer));

    Ok(metadata_layer)
}

pub async fn get_annotation_layer_paths(
    id: u32,
    conn: Arc<Mutex<Connection>>,
) -> Result<Vec<(String, PathBuf)>> {
    let conn_lock = conn.lock().unwrap();
    let mut stmt = conn_lock.prepare(
        r#"
            SELECT directory_path
            FROM images
            WHERE id = ?1;
        "#,
    )?;

    let parent_directory_path =
        stmt.query_row([id], |row| Ok(PathBuf::from(row.get::<_, String>(0)?)))?;

    let mut stmt = conn_lock.prepare(
        r#"
            SELECT tag
            FROM annotation_layer
            WHERE image_id = ?1;
        "#,
    )?;

    let annotation_layers = stmt
        .query_map([id], |row| {
            let tag = row.get::<_, String>(0)?;
            Ok((tag.clone(), parent_directory_path.join(tag + ".json")))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    #[cfg(feature = "log.database")]
    log(
        &format!("GET <Annotation Paths: {}>", id),
        Some(&annotation_layers),
    );

    Ok(annotation_layers)
}

pub async fn get(id: u32, conn: Arc<Mutex<Connection>>) -> Result<ImageState> {
    let paths = get_paths(id, Arc::clone(&conn)).await?;
    let metadata_layers = get_metadata_layers(id, conn).await?;

    let state = ImageState {
        directory_path: paths.directory_path.into(),
        image_name: paths.image_name.into(),
        store_name: paths.store_name.into(),
        annotations_name: paths.annotations_name,
        metadata_layers,
    };

    Ok(state)
}

pub async fn remove(id: u32, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let conn_lock = conn.lock().unwrap();
    conn_lock.execute(
        r#"
            DELETE FROM images WHERE id = ?1;
        "#,
        [id],
    )?;

    #[cfg(feature = "log.database")]
    log::<()>(&format!("DELETE <Image: {}>", id), None);

    Ok(())
}

#[cfg(feature = "log.database")]
fn log<T: Debug>(operation: &str, result: Option<&T>) {
    print!("Database <{}>", operation);
    if let Some(result) = result {
        print!(": {:#?}", result);
    }
    println!("\n");
}
