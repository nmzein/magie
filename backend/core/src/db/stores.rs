use crate::db::prelude::*;
use crate::{
    constants::{BIN_ID, ROOT_ID},
    types::{
        database::Interface,
        fs::{Asset, Directory, Entry},
    },
};

pub fn create(registry_conn: &Connection, r#type: &Interface, name: &str) -> Result<()> {
    // Create the store in the registry.
    let Ok(_) = registry_conn.execute(
        "
            INSERT INTO stores (type, name)
            VALUES (?1, ?2);
        ",
        (&r#type, name),
    ) else {
        return Ok(());
    };

    let store_id = u32::try_from(registry_conn.last_insert_rowid())?;

    let path = crate::io::create_store(store_id)?;
    let url = crate::io::create_store_database(store_id)?;

    let mut conn = Connection::open(&url)?;
    let transaction = conn.transaction()?;

    transaction.execute(
        r#"
            CREATE TABLE IF NOT EXISTS id_counter (next_id INTEGER NOT NULL);
        "#,
        (),
    )?;

    transaction.execute(
        r#"
            INSERT
            OR IGNORE INTO id_counter (next_id)
            VALUES (1);
        "#,
        (),
    )?;

    transaction.execute(r#"
        CREATE TABLE IF NOT EXISTS directories (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            predeletion_parent_id INTEGER,
            name TEXT NOT NULL,
            FOREIGN KEY (parent_id) REFERENCES directories (id) ON DELETE CASCADE,
            UNIQUE (parent_id, name) -- Enforce that for each parent directory, there is only one directory with a given name.
        );
    "#, ())?;

    transaction.execute(r#"
        CREATE TABLE IF NOT EXISTS images (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER NOT NULL,
            predeletion_parent_id INTEGER,
            name TEXT NOT NULL,
            created_at DATETIME,
            updated_at DATETIME,
            decoder TEXT,
            encoder TEXT NOT NULL,
            generator TEXT,
            uploaded_image_extension TEXT NOT NULL,
            uploaded_annotations_extension TEXT,
            FOREIGN KEY (parent_id) REFERENCES directories (id) ON DELETE CASCADE,
            UNIQUE (parent_id, name) -- Enforce that for each parent directory, there is only one file with a given name.
        );
    "#, ())?;

    transaction.execute(
        r#"
        CREATE TABLE IF NOT EXISTS metadata_layer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            image_id INTEGER NOT NULL,
            level INTEGER NOT NULL,
            cols INTEGER NOT NULL,
            rows INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
            UNIQUE (image_id, level)
        );
    "#,
        (),
    )?;

    transaction.execute(
        r#"
        CREATE TABLE IF NOT EXISTS annotation_layer (
            id INTEGER NOT NULL,
            image_id INTEGER NOT NULL,
            tag TEXT NOT NULL,
            colour TEXT NOT NULL,
            FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
            UNIQUE (image_id, tag)
        );
    "#,
        (),
    )?;

    // Create virtual root directory.
    transaction.execute(
        "INSERT INTO directories (id, name) VALUES (?1, ?2);",
        (ROOT_ID, ""),
    )?;

    // Create bin directory.
    transaction.execute(
        "INSERT INTO directories (id, name, parent_id) VALUES (?1, ?2, ?3);",
        (BIN_ID, "Bin", ROOT_ID),
    )?;

    transaction.commit()?;

    // Update with path and url.
    registry_conn.execute(
        "
            UPDATE stores
            SET path = ?1,
                url = ?2
            WHERE id = ?3;
        ",
        (path.to_str(), url, store_id),
    )?;

    Ok(())
}

/// Fetches all directories and subdirectories for a store and builds object.
pub fn get(dbm: &DatabaseManager, store_id: u32) -> Result<Vec<Entry>> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "SELECT d.id, d.name, d.parent_id,
                COALESCE(GROUP_CONCAT(DISTINCT f.id), '') AS file_ids,
                COALESCE(GROUP_CONCAT(DISTINCT sub_d.id), '') AS subdir_ids
         FROM directories d
         LEFT JOIN images f ON d.id = f.parent_id
         LEFT JOIN directories sub_d ON d.id = sub_d.parent_id
         GROUP BY d.id;",
    )?;

    let directories: Vec<Directory> = stmt
        .query_map([], |row| {
            let file_ids: String = row.get(3)?;
            let subdir_ids: String = row.get(4)?;
            let mut children: Vec<u32> = file_ids
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            children.extend(subdir_ids.split(',').filter_map(|s| s.parse::<u32>().ok()));
            Ok(Directory {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get::<_, Option<u32>>(2)?,
                children,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut stmt = conn.prepare_cached("SELECT id, name, parent_id FROM images;")?;
    let assets: Vec<Asset> = stmt
        .query_map([], |row| {
            Ok(Asset {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get::<_, Option<u32>>(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut result: Vec<Entry> = directories.into_iter().map(Entry::Directory).collect();
    result.extend(assets.into_iter().map(Entry::Asset));

    Ok(result)
}

/// Fetches all directories and subdirectories starting from a given directory_id.
pub fn get_images_below(
    dbm: &DatabaseManager,
    store_id: u32,
    directory_id: u32,
) -> Result<Vec<Asset>> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "WITH RECURSIVE directory_tree AS (
                SELECT id FROM directories WHERE id = ?1
                UNION ALL
                SELECT d.id FROM directories d
                JOIN directory_tree dt ON d.parent_id = dt.id
            )
            SELECT f.id, f.name, f.parent_id
            FROM images f
            WHERE f.parent_id IN (SELECT id FROM directory_tree);",
    )?;

    let assets = stmt
        .query_map([directory_id], |row| {
            Ok(Asset {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get::<_, Option<u32>>(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(assets)
}
