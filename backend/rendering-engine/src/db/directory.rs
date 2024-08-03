use crate::db::common::*;
use crate::types::MoveMode;
use std::path::PathBuf;
use std::sync::MutexGuard;

pub fn r#move(id: u32, target_id: u32, mode: MoveMode, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let conn = conn.lock().unwrap();

    let parent_id: u32 = conn.query_row(
        r#"
            SELECT parent_id
            FROM directories
            WHERE id = ?1;
        "#,
        [id],
        |row| row.get(0),
    )?;

    // Fill the space that would be created from moving this directory.
    let _ = fill_space(parent_id, &conn);

    // Make space in the bin for this directory.
    let target_rgt = make_space(target_id, &conn)?;

    match mode {
        MoveMode::Regular => {
            conn.execute(
                r#"
                UPDATE directories
                SET parent_id = ?1,
                    lft = ?2,
                    rgt = ?3
                WHERE id = ?4;
            "#,
                [target_id, target_rgt, target_rgt + 1, id],
            )?;
        }
        MoveMode::SoftDelete => {
            conn.execute(
                r#"
                UPDATE directories
                SET predeletion_parent_id = ?1,
                    parent_id = ?2,
                    lft = ?3,
                    rgt = ?4
                WHERE id = ?5;
            "#,
                [parent_id, target_id, target_rgt, target_rgt + 1, id],
            )?;
        }
    }

    Ok(())
}

fn fill_space(parent_id: u32, conn: &MutexGuard<Connection>) -> Result<u32> {
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

    // Update the rgt values of the parent, ancestors, and siblings and their children.
    conn.execute(
        r#"
                UPDATE directories
                SET rgt = rgt - 2
                WHERE rgt >= ?1;
            "#,
        [parent_rgt],
    )?;

    // Update the lft values of the siblings and their children.
    conn.execute(
        r#"
                UPDATE directories
                SET lft = lft - 2
                WHERE lft > ?1;
            "#,
        [parent_rgt],
    )?;

    return Ok(parent_rgt);
}

// Need to "make space" by adding 2 to the rgt values
// of the parent and ancestors. Also need to "shift"
// sibling nodes by adding 2 to their lft and rgt values.
fn make_space(parent_id: u32, conn: &MutexGuard<Connection>) -> Result<u32> {
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
            WHERE rgt >= ?1;
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

    return Ok(parent_rgt);
}

pub fn insert(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let conn = conn.lock().unwrap();

    let parent_rgt = make_space(parent_id, &conn)?;

    // Insert the new directory.
    conn.execute(
        r#"
            INSERT INTO directories (name, parent_id, lft, rgt)
            VALUES (?1, ?2, ?3, ?4);
        "#,
        [
            name,
            &parent_id.to_string(),
            &(parent_rgt).to_string(),
            &(parent_rgt + 1).to_string(),
        ],
    )?;

    #[cfg(feature = "log.database")]
    log(&format!("INSERT <Directory: {parent_id}/{name}>"), None);

    Ok(())
}

/// Returns true if an directory with the given name is a child of directory with given id.
pub fn exists(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<Option<PathBuf>> {
    let conn = conn.lock().unwrap();
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

pub fn path_internal(id: u32, conn: &MutexGuard<Connection>) -> Result<PathBuf> {
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
