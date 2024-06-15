use crate::db::common::*;
use std::path::PathBuf;

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
    log(&format!("GET <Directory: {}>", id), Some(&path));

    Ok(path)
}
