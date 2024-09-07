use crate::db::common::*;
use crate::types::MoveMode;
use rusqlite::Transaction;
use std::path::PathBuf;

// Makes `width` number of space under a given directory.
fn make_space(id: u32, width: u32, transaction: &Transaction) -> Result<u32> {
    // Get the rgt value of the directory.
    let rgt = transaction.query_row(
        r#"
            SELECT rgt
            FROM directories
            WHERE id = ?1;
        "#,
        [id],
        |row| row.get(0),
    )?;

    // Update the rgt values of the directory (hence the =), its parent, ancestors, and siblings and their children.
    transaction.execute(
        r#"
            UPDATE directories
            SET rgt = rgt + ?1
            WHERE rgt >= ?2;
        "#,
        [width, rgt],
    )?;

    // Update the lft values of the siblings and their children.
    transaction.execute(
        r#"
            UPDATE directories
            SET lft = lft + ?1
            WHERE lft > ?2;
        "#,
        [width, rgt],
    )?;

    return Ok(rgt + width);
}

fn shrink_space(width: u32, threshold: u32, transaction: &Transaction) -> Result<()> {
    // Update the lft values of the siblings and their children.
    transaction.execute(
        r#"
            UPDATE directories
            SET lft = lft - ?1
            WHERE lft > ?2 AND rgt > ?2;
        "#,
        [width, threshold],
    )?;

    // Update the rgt values of the parent, ancestors, and siblings and their children.
    transaction.execute(
        r#"
            UPDATE directories
            SET rgt = rgt - ?1
            WHERE rgt > ?2;
        "#,
        [width, threshold],
    )?;

    return Ok(());
}

pub fn insert(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let mut conn = conn.lock().unwrap();
    let transaction = conn.transaction()?;

    // Get the rgt value of the parent.
    let parent_rgt: u32 = make_space(parent_id, 2, &transaction)?;

    // Insert the new directory.
    transaction.execute(
        r#"
            INSERT INTO directories (name, parent_id, lft, rgt)
            VALUES (?1, ?2, ?3, ?4);
        "#,
        (name, &parent_id, &(parent_rgt - 2), &(parent_rgt - 1)),
    )?;

    let _ = transaction.commit();

    #[cfg(feature = "log.database")]
    log(&format!("INSERT <Directory: {parent_id}/{name}>"), None);

    Ok(())
}

pub fn delete(id: u32, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let mut conn = conn.lock().unwrap();
    let transaction = conn.transaction()?;

    // Get the rgt value of the directory.
    let (lft, rgt) = transaction.query_row(
        r#"
            SELECT lft, rgt
            FROM directories
            WHERE id = ?1;
        "#,
        [id],
        |row| Ok((row.get::<_, u32>(0)?, row.get::<_, u32>(1)?)),
    )?;

    // Shrink the space that would be left behind after deleting this directory.
    let _ = shrink_space(rgt - lft + 1, rgt, &transaction);

    // Delete the directory.
    transaction.execute(
        r#"
            DELETE FROM directories
            WHERE id = ?1;
        "#,
        [id],
    )?;

    let _ = transaction.commit();

    #[cfg(feature = "log.database")]
    log(&format!("DELETE <Directory: {id}>"), None);

    Ok(())
}

pub fn r#move(
    id: u32,
    destination_id: u32,
    mode: MoveMode,
    conn: Arc<Mutex<Connection>>,
) -> Result<()> {
    let mut conn = conn.lock().unwrap();
    let transaction = conn.transaction()?;

    let width: u32 = transaction.query_row(
        r#"
            SELECT rgt - lft
            FROM directories
            WHERE id = ?1;
        "#,
        [id],
        |row| row.get(0),
    )?;

    // Make space under the destination directory for this directory and its children.
    let destination_rgt = make_space(destination_id, width + 1, &transaction)?;

    let target_lft = destination_rgt - 1 - width;
    let target_rgt = destination_rgt - 1;

    // The current left and right values of the directory after space was made.
    let (lft, rgt, parent_id): (u32, u32, u32) = transaction.query_row(
        r#"
            SELECT lft, rgt, parent_id
            FROM directories
            WHERE id = ?1;
        "#,
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;

    let offset: i32 = target_rgt as i32 - rgt as i32;

    match mode {
        MoveMode::Regular => {
            // Update the directory.
            transaction.execute(
                r#"
                    UPDATE directories
                    SET parent_id = ?1,
                        lft = ?2,
                        rgt = ?3
                    WHERE id = ?4;
                "#,
                (destination_id, target_lft, target_rgt, id),
            )?;

            // Update the children of the directory.
            transaction.execute(
                r#"
                    UPDATE directories
                    SET lft = lft + ?1,
                        rgt = rgt + ?1
                    WHERE lft > ?2 AND rgt < ?3;
                "#,
                (offset, lft, rgt),
            )?;
        }
        MoveMode::SoftDelete => {
            // TODO: Have DELETED field for directory and children.

            // Mark the directory as deleted.
            transaction.execute(
                r#"
                    UPDATE directories
                    SET predeletion_parent_id = ?1,
                        parent_id = ?2,
                        lft = ?3,
                        rgt = ?4
                    WHERE id = ?5;
                "#,
                (parent_id, destination_id, target_lft, target_rgt, id),
            )?;

            // Update the children of the directory.
            transaction.execute(
                r#"
                    UPDATE directories
                    SET lft = lft + ?1,
                        rgt = rgt + ?1
                    WHERE lft > ?2 AND rgt < ?3;
                "#,
                (offset, lft, rgt),
            )?;
        }
    }

    // Shrink the space that would be left behind after moving this directory and its children.
    let _ = shrink_space(width + 1, rgt, &transaction);

    let _ = transaction.commit();

    Ok(())
}

/// Returns true if an directory with the given name is a child of directory with given id.
pub fn exists(parent_id: u32, name: &str, conn: Arc<Mutex<Connection>>) -> Result<Option<PathBuf>> {
    let exists: bool;
    {
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"
                SELECT 1 FROM directories WHERE name = ?1 AND parent_id = ?2;
            "#,
        )?;

        exists = stmt.exists(&[name, &parent_id.to_string()])?;
    }

    #[cfg(feature = "log.database")]
    log(
        &format!("CONTAINS <Directory: {parent_id}/{name}>"),
        Some(&exists),
    );

    if !exists {
        // Return the would-be path for the new directory.
        Ok(Some(path(parent_id, Arc::clone(&conn))?.join(name)))
    } else {
        Ok(None)
    }
}

pub fn path(id: u32, conn: Arc<Mutex<Connection>>) -> Result<PathBuf> {
    let conn = conn.lock().unwrap();

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
