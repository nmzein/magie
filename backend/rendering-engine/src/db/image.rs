use super::common::*;
use anyhow::anyhow;
use chrono::Utc;
use shared::types::{AnnotationLayer, ImageProperties, MetadataLayer};

/// Returns the path of an image given its id and store id.
pub fn path(store_id: u32, image_id: u32) -> Result<PathBuf> {
    Ok(DB
        .stores
        .get(&store_id)
        .ok_or(anyhow!("Requested store does not exist."))?
        .properties
        .image(image_id))
}

#[wrap_with_store(r#move)]
pub fn r#move_<C>(conn: C, image_id: u32, destination_id: u32) -> Result<()>
where
    C: Deref<Target = Connection>,
{
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

#[wrap_with_store(insert)]
pub fn insert_<C>(
    mut conn: C,
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
) -> Result<()>
where
    C: DerefMut<Target = Connection>,
{
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

#[wrap_with_store(delete)]
pub fn delete_<C>(conn: C, image_id: u32) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    let mut stmt = conn.prepare_cached("DELETE FROM images WHERE id = ?1;")?;
    stmt.execute([image_id])?;

    Ok(())
}

#[wrap_with_store(properties)]
pub fn properties_<C>(conn: C, image_id: u32) -> Result<ImageProperties>
where
    C: Deref<Target = Connection>,
{
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
