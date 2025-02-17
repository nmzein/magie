use super::common::*;
use anyhow::anyhow;
use rusqlite_migration::{Migrations, M};
use shared::{
    constants::{BIN_ID, ROOT_ID},
    types::{Directory, File, FileSystemEntry},
};
use std::fs;

pub fn bin(store_id: u32) -> Result<PathBuf> {
    Ok(DB
        .stores
        .get(&store_id)
        .ok_or(anyhow!("Requested store does not exist."))?
        .properties
        .bin())
}

#[wrap_with_store(create)]
pub fn create_<C>(registry_conn: C, r#type: DatabaseType, name: &str, path: PathBuf) -> Result<()>
where
    C: Deref<Target = Connection>,
{
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

    let store_id = registry_conn.last_insert_rowid() as u32;
    let location = format!("{STORES_DATABASE_PATH_PREFIX}{store_id}.sqlite");
    let path = path.join(format!("s{store_id}"));
    let url = format!("{STORES_DATABASE_URL_PREFIX}{store_id}.sqlite");

    fs::File::create(location)?;
    fs::create_dir_all(&path)?;

    let mut conn = Connection::open(&url)?;

    let migrations = Migrations::new(vec![M::up(include_str!("../../../../databases/store.sql"))]);
    migrations.to_latest(&mut conn).unwrap();

    // Create virtual root directory.
    conn.execute(
        "INSERT INTO directories (id, name) VALUES (?1, ?2);",
        (ROOT_ID, ""),
    )?;

    // Create bin directory.
    conn.execute(
        "INSERT INTO directories (id, name, parent_id) VALUES (?1, ?2, ?3);",
        (BIN_ID, "Bin", ROOT_ID),
    )?;

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
#[wrap_with_store(get)]
pub fn get_<C>(conn: C) -> Result<Vec<FileSystemEntry>>
where
    C: Deref<Target = Connection>,
{
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
    let files: Vec<File> = stmt
        .query_map([], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get::<_, Option<u32>>(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut result: Vec<FileSystemEntry> = directories
        .into_iter()
        .map(FileSystemEntry::Directory)
        .collect();
    result.extend(files.into_iter().map(FileSystemEntry::File));

    Ok(result)
}

/// Fetches all directories and subdirectories starting from a given directory_id.
#[wrap_with_store(get_images_below)]
pub fn get_images_below_<C>(conn: C, directory_id: u32) -> Result<Vec<File>>
where
    C: Deref<Target = Connection>,
{
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

    let files = stmt
        .query_map([directory_id], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get::<_, Option<u32>>(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(files)
}
