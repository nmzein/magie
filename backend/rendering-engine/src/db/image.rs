use crate::db::common::*;
use crate::types::{AnnotationLayer as OutAnnotationLayer, ImageProperties};
use rusqlite::named_params;
use shared::structs::{AnnotationLayer as InAnnotationLayer, MetadataLayer};
use std::path::PathBuf;

pub fn insert(
    parent_id: u32,
    name: &str,
    upl_img_ext: &str,
    upl_img_fmt: &str,
    enc_img_fmt: &str,
    annotations_ext: Option<&str>,
    metadata_layers: Vec<MetadataLayer>,
    annotation_layers: Vec<InAnnotationLayer>,
) -> Result<()> {
    let mut conn = RDB.conn.lock().unwrap();
    let transaction = conn.transaction()?;

    // TODO: Remove hardcoding.
    transaction.execute(
        "
            INSERT INTO images (parent_id, name, upl_img_ext, upl_img_fmt, enc_img_ext, enc_img_fmt, upl_anno_ext)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
        ",
        (parent_id, name, upl_img_ext,  upl_img_fmt, "zarr", enc_img_fmt, annotations_ext),
    )?;

    let image_id = transaction.last_insert_rowid();

    #[cfg(feature = "log.database")]
    log::<()>(&format!("INSERT <Image: {image_id}>"), None);

    for m in metadata_layers {
        transaction.execute(
            "
                INSERT INTO metadata_layer (image_id, level, cols, rows, width, height)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6);
            ",
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
            "
                INSERT INTO annotation_layer (image_id, tag)
                VALUES (?1, ?2);
            ",
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

pub fn delete(id: u32) -> Result<()> {
    let conn = RDB.conn.lock().unwrap();
    conn.execute(
        "
            DELETE FROM images WHERE id = ?1;
        ",
        [id],
    )?;

    #[cfg(feature = "log.database")]
    log::<()>(&format!("DELETE <Image: {id}>"), None);

    Ok(())
}

/// Returns true if an image with the given name is a child of directory with given id.
pub fn exists(parent_id: u32, name: &str) -> Result<bool> {
    let conn = RDB.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "
            SELECT 1 FROM images WHERE name = ?1 AND parent_id = ?2;
        ",
    )?;

    let exists = stmt.exists([name, &parent_id.to_string()])?;

    #[cfg(feature = "log.database")]
    log(
        &format!("CONTAINS <Image: {parent_id}/{name}>"),
        Some(&exists),
    );

    Ok(exists)
}

/// Returns the name and path of an image given its id.
pub fn get(id: u32) -> Result<(String, PathBuf)> {
    let returned: (String, u32);

    {
        let conn = RDB.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "
            SELECT name, parent_id
            FROM images
            WHERE id = ?1;
        ",
        )?;

        returned = stmt.query_row([id], |row| Ok((row.get(0)?, row.get(1)?)))?;
    }

    let (name, parent_id) = returned;
    let path = crate::db::directory::path(parent_id)?.join(&name);

    #[cfg(feature = "log.database")]
    log(&format!("GET <Image: {path:?}>"), Some(&path));

    Ok((name, path))
}

pub fn properties(id: u32) -> Result<ImageProperties> {
    let conn = RDB.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "
            SELECT level, cols, rows, width, height
            FROM metadata_layer
            WHERE image_id = ?1
            ORDER BY level ASC;
        ",
    )?;

    let metadata_layers = stmt
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

    let mut stmt = conn.prepare(
        "
                SELECT id, tag
                FROM annotation_layer
                WHERE image_id = ?1;
            ",
    )?;

    let annotation_layers = stmt
        .query_map([id], |row| {
            Ok(OutAnnotationLayer::new(row.get(0)?, row.get(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    #[cfg(feature = "log.database")]
    log(&format!("GET <Metadata: {id}>"), Some(&metadata_layer));

    Ok(ImageProperties {
        metadata: metadata_layers,
        annotations: annotation_layers,
    })
}

pub fn get_annotation_layer_path(image_id: u32, annotation_layer_id: u32) -> Result<PathBuf> {
    let parent_directory_path = get(image_id)?.1;
    let conn = RDB.conn.lock().unwrap();

    let mut stmt = conn.prepare(
        "
            SELECT tag
            FROM annotation_layer
            WHERE image_id = :image_id
            AND id = :annotation_layer_id;
        ",
    )?;

    let path = stmt.query_row(
        named_params! { ":image_id": image_id, ":annotation_layer_id": annotation_layer_id },
        |row| {
            let tag = row.get::<_, String>(0)?;
            Ok(parent_directory_path.join(tag + ".glb"))
        },
    )?;

    #[cfg(feature = "log.database")]
    log(
        &format!("GET <Annotation Layer Path: {image_id}:{path}>"),
        Some(&layer),
    );

    Ok(path)
}

pub fn r#move(id: u32, destination_id: u32) -> Result<()> {
    let conn = RDB.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "
            UPDATE images
            SET parent_id = ?1
            WHERE id = ?2;
        ",
    )?;

    stmt.execute([destination_id, id])?;

    #[cfg(feature = "log.database")]
    log(
        &format!("MOVE <Image: {id}> to <Directory: {destination_id}>"),
        None,
    );

    Ok(())
}

pub fn path(id: u32) -> Result<PathBuf> {
    let (parent_id, name): (u32, String);
    {
        let conn = RDB.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "
            SELECT parent_id, name
            FROM images
            WHERE id = ?1;
            ",
        )?;

        (parent_id, name) = stmt.query_row([id], |row| Ok((row.get(0)?, row.get(1)?)))?;
    }

    let parent_directory_path = crate::db::directory::path(parent_id)?;
    let path = parent_directory_path.join(name);

    #[cfg(feature = "log.database")]
    log(&format!("GET <Image Path: {id}>"), Some(&path));

    Ok(path)
}
