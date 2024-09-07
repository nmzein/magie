use crate::db::common::*;
use crate::types::MetadataLayer;
use shared::structs::AnnotationLayer;
use std::path::PathBuf;

pub fn insert(
    parent_id: u32,
    name: &str,
    upl_img_ext: &str,
    annotations_ext: Option<&str>,
    metadata_layers: Vec<MetadataLayer>,
    annotation_layers: Vec<AnnotationLayer>,
    conn: Arc<Mutex<Connection>>,
) -> Result<()> {
    let mut conn = conn.lock().unwrap();
    let transaction = conn.transaction()?;

    // TODO: Remove hardcoding.
    transaction.execute(
        r#"
            INSERT INTO images (parent_id, name, upl_img_ext, enc_img_ext, upl_img_fmt, enc_img_fmt, upl_anno_ext)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
        "#,
        (parent_id, name, upl_img_ext, "zarr", upl_img_ext, "omezarr", annotations_ext),
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

pub fn delete(id: u32, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let conn = conn.lock().unwrap();
    conn.execute(
        r#"
            DELETE FROM images WHERE id = ?1;
        "#,
        [id],
    )?;

    #[cfg(feature = "log.database")]
    log::<()>(&format!("DELETE <Image: {id}>"), None);

    Ok(())
}

/// Returns true if an image with the given name is a child of directory with given id.
pub fn exists(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<bool> {
    let conn: std::sync::MutexGuard<'_, Connection> = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        r#"
            SELECT 1 FROM images WHERE name = ?1 AND parent_id = ?2;
        "#,
    )?;

    let exists = stmt.exists(&[name, &parent_id.to_string()])?;

    #[cfg(feature = "log.database")]
    log(
        &format!("CONTAINS <Image: {parent_id}/{name}>"),
        Some(&exists),
    );

    Ok(exists)
}

/// Returns the name and path of an image given its id.
pub fn get(id: u32, conn: Arc<Mutex<Connection>>) -> Result<(String, PathBuf)> {
    let returned: (String, u32);

    {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
            SELECT name, parent_id
            FROM images
            WHERE id = ?1;
        "#,
        )?;

        returned = stmt.query_row([id], |row| Ok((row.get(0)?, row.get(1)?)))?;
    }

    let (name, parent_id) = returned;
    let path = crate::db::directory::path(parent_id, Arc::clone(&conn))?.join(&name);

    #[cfg(feature = "log.database")]
    log(&format!("GET <Image: {path:?}>"), Some(&path));

    Ok((name, path))
}

pub fn get_metadata_layers(id: u32, conn: Arc<Mutex<Connection>>) -> Result<Vec<MetadataLayer>> {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
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
    log(&format!("GET <Metadata: {id}>"), Some(&metadata_layer));

    Ok(metadata_layer)
}

pub fn get_annotation_layer_paths(
    id: u32,
    conn: Arc<Mutex<Connection>>,
) -> Result<Vec<(String, PathBuf)>> {
    let parent_directory_path = get(id, Arc::clone(&conn))?.1;
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare(
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
        &format!("GET <Annotation Paths: {id}>"),
        Some(&annotation_layers),
    );

    Ok(annotation_layers)
}
