use super::common::*;
use crate::constants::BIN_ID;

pub fn insert(dbm: &DatabaseManager, store_id: u32, parent_id: u32, name: &str) -> Result<u32> {
    let id = counter(&dbm, store_id)?;
    let conn = dbm.store(store_id)?;

    let mut stmt =
        conn.prepare_cached("INSERT INTO directories (id, name, parent_id) VALUES (?1, ?2, ?3);")?;
    stmt.execute((id, name, parent_id))?;

    Ok(id)
}

pub fn delete(dbm: &DatabaseManager, store_id: u32, directory_id: u32) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached("DELETE FROM directories WHERE id = ?1;")?;
    stmt.execute([directory_id])?;

    Ok(())
}

pub fn soft_delete(dbm: &DatabaseManager, store_id: u32, directory_id: u32) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            UPDATE directories
            SET predeletion_parent_id = parent_id,
                parent_id = ?1
            WHERE id = ?2;
        ",
    )?;
    stmt.execute((BIN_ID, directory_id))?;

    Ok(())
}

pub fn r#move(
    dbm: &DatabaseManager,
    store_id: u32,
    directory_id: u32,
    destination_id: u32,
) -> Result<()> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            UPDATE directories
            SET parent_id = ?1
            WHERE id = ?2;
        ",
    )?;
    stmt.execute((destination_id, directory_id))?;

    Ok(())
}

pub fn is_within(
    dbm: &DatabaseManager,
    store_id: u32,
    descendant_id: u32,
    ancestor_id: u32,
) -> Result<bool> {
    let conn = dbm.store(store_id)?;

    let mut stmt = conn.prepare_cached(
        "
            WITH RECURSIVE parent_chain AS (
                SELECT id, parent_id
                FROM directories
                WHERE id = ?1

                UNION ALL

                SELECT d.id, d.parent_id
                FROM directories d
                INNER JOIN parent_chain pc ON d.id = pc.parent_id
            )
            SELECT 1
            FROM parent_chain
            WHERE parent_id = ?2
            LIMIT 1;
            ",
    )?;

    let exists = stmt.exists([descendant_id, ancestor_id])?;

    Ok(exists)
}
