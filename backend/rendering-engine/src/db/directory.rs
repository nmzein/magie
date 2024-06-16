use crate::db::common::*;
use std::path::PathBuf;

pub fn insert(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let conn = conn.lock().unwrap();

    // Need to "make space" by adding 2 to the rgt values
    // of the parent and ancestors. Also need to "shift"
    // sibling nodes by adding 2 to their lft and rgt values.

    // Get the rgt value of the parent.
    let parent_rgt: u32 = conn.query_row(
        r#"
            SELECT rgt
            FROM directories
            WHERE id = ?1;
        "#,
        [parent_id],
        |row| row.get(0),
    )?;

    // Update the rgt values of the parent, ancestors, and siblings.
    conn.execute(
        r#"
            UPDATE directories
            SET rgt = rgt + 2
            WHERE rgt > ?1;
        "#,
        [parent_rgt],
    )?;

    // Update the lft values of the siblings.
    conn.execute(
        r#"
            UPDATE directories
            SET lft = lft + 2
            WHERE lft > ?1;
        "#,
        [parent_rgt],
    )?;

    // Insert the new directory.
    conn.execute(
        r#"
            INSERT INTO directories (name, parent_id, lft, rgt)
            VALUES (?1, ?2, ?3, ?4);
        "#,
        [
            name,
            &parent_id.to_string(),
            &(parent_rgt + 1).to_string(),
            &(parent_rgt + 2).to_string(),
        ],
    )?;

    #[cfg(feature = "log.database")]
    log(&format!("INSERT <Directory: {parent_id}/{name}>"), None);

    Ok(())
}

/// Returns true if an directory with the given name is a child of directory with given id.
pub fn exists(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<Option<PathBuf>> {
    let conn: std::sync::MutexGuard<'_, Connection> = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        r#"
            SELECT 1 FROM directories WHERE name = ?1 AND parent_id = ?2;
        "#,
    )?;

    let exists = stmt.exists(&[name, &parent_id.to_string()])?;

    #[cfg(feature = "log.database")]
    log(
        &format!("CONTAINS <Directory: {parent_id}/{name}>"),
        Some(&exists),
    );

    if !exists {
        // Return the would-be path for the new directory.
        Ok(Some(path_internal(parent_id, &conn)?.join(name)))
    } else {
        Ok(None)
    }
}

// TODO: Remove indirection.
pub fn path(id: u32, conn: Arc<Mutex<Connection>>) -> Result<PathBuf> {
    let conn = conn.lock().unwrap();
    path_internal(id, &conn)
}

pub fn path_internal(id: u32, conn: &std::sync::MutexGuard<Connection>) -> Result<PathBuf> {
    // Combine the two queries into one
    let mut stmt = conn.prepare(
        r#"
            WITH dir AS (
                SELECT lft, rgt
                FROM directories
                WHERE id = ?1
            )
            SELECT d.name
            FROM directories d, dir
            WHERE d.lft <= dir.lft AND d.rgt >= dir.rgt
            ORDER BY d.lft;
        "#,
    )?;

    // Initialize an empty PathBuf
    let mut path = PathBuf::new();

    // Execute the combined query and build the path directly
    stmt.query_map([id], |row| {
        let segment: String = row.get(0)?;
        // Append each segment to the PathBuf
        path.push(segment);
        Ok(())
    })?
    .collect::<Result<Vec<_>, _>>()?;

    #[cfg(feature = "log.database")]
    log(&format!("GET <Directory: {id}>"), Some(&path));

    Ok(path)
}
