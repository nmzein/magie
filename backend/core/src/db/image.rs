use crate::constants::{BIN_ID, IMAGE_NAME, THUMBNAIL_NAME};
use crate::db::common::*;
use chrono::Utc;
use shared::types::{AnnotationLayer, ImageProperties, MetadataLayer};

/// Returns the path of an image given its id and store id.
// pub fn path(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<PathBuf> {
//     Ok(dbm
//         .store_properties(store_id)?
//         .path
//         .join(format!("i{image_id}")))
// }

pub fn image_path(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<PathBuf> {
    Ok(dbm
        .store_properties(store_id)?
        .path
        .join(format!("i{image_id}/{IMAGE_NAME}")))
}

pub fn annotation_path(
    dbm: &DatabaseManager,
    store_id: u32,
    image_id: u32,
    annotation_layer_id: u32,
) -> Result<PathBuf> {
    Ok(dbm
        .store_properties(store_id)?
        .path
        .join(format!("i{image_id}/a{annotation_layer_id}.glb")))
}

pub fn thumbnail_path(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<PathBuf> {
    Ok(dbm
        .store_properties(store_id)?
        .path
        .join(format!("i{image_id}/{THUMBNAIL_NAME}")))
}

pub fn get_parent(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<u32> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            SELECT parent_id
            FROM images
            WHERE id = ?1;
        ",
    )?;

    let parent_id = stmt.query_row([image_id], |row| row.get(0))?;

    Ok(parent_id)
}

pub fn r#move(
    dbm: &DatabaseManager,
    store_id: u32,
    image_id: u32,
    destination_id: u32,
) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            UPDATE images
            SET parent_id = ?1
            WHERE id = ?2;
        ",
    )?;

    stmt.execute([destination_id, image_id])?;

    Ok(())
}

pub fn soft_delete(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            UPDATE images
            SET predeletion_parent_id = parent_id,
                parent_id = ?1
            WHERE id = ?2;
        ",
    )?;

    stmt.execute([BIN_ID, image_id])?;

    Ok(())
}

pub fn insert(
    dbm: &DatabaseManager,
    store_id: u32,
    image_id: u32,
    parent_id: u32,
    name: &str,
    decoder: &str,
    encoder: &str,
    generator: Option<&str>,
    uploaded_image_extension: &str,
    uploaded_annotations_extension: Option<&str>,
    metadata_layers: Vec<MetadataLayer>,
    annotation_layers: Vec<AnnotationLayer>,
) -> Result<()> {
    let mut conn = dbm.store(store_id)?;

    let transaction = conn.transaction()?;

    {
        let mut stmt =  transaction.prepare_cached(
        "
            INSERT INTO images (id, parent_id, name, created_at, updated_at, decoder, encoder, generator, uploaded_image_extension, uploaded_annotations_extension)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);
        ",
    )?;

        let now = Utc::now().to_rfc3339();

        stmt.execute((
            image_id,
            parent_id,
            name,
            now.clone(),
            now,
            decoder,
            encoder,
            generator,
            uploaded_image_extension,
            uploaded_annotations_extension,
        ))?;
    }

    {
        let mut stmt = transaction.prepare_cached(
            "
            INSERT INTO metadata_layer (image_id, level, cols, rows, width, height)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6);
        ",
        )?;

        for m in metadata_layers {
            stmt.execute((image_id, m.level, m.cols, m.rows, m.width, m.height))?;
        }
    }

    {
        let mut stmt = transaction.prepare_cached(
            "
            INSERT INTO annotation_layer (id, image_id, tag, colour)
            VALUES (?1, ?2, ?3, ?4);
        ",
        )?;

        for a in annotation_layers {
            stmt.execute((a.id, image_id, a.tag, a.fill))?;
        }
    }

    transaction.commit()?;

    Ok(())
}

pub fn delete(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached("DELETE FROM images WHERE id = ?1;")?;
    stmt.execute([image_id])?;

    Ok(())
}

pub fn properties(dbm: &DatabaseManager, store_id: u32, image_id: u32) -> Result<ImageProperties> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            SELECT level, cols, rows, width, height
            FROM metadata_layer
            WHERE image_id = ?1
            ORDER BY level ASC;
        ",
    )?;

    let metadata_layers = stmt
        .query_map([image_id], |row| {
            Ok(MetadataLayer {
                level: row.get(0)?,
                cols: row.get(1)?,
                rows: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut stmt = conn.prepare_cached(
        "
            SELECT id, tag, colour
            FROM annotation_layer
            WHERE image_id = ?1;
        ",
    )?;

    let annotation_layers = stmt
        .query_map([image_id], |row| {
            Ok(AnnotationLayer::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ImageProperties {
        metadata: metadata_layers,
        annotations: annotation_layers,
    })
}
