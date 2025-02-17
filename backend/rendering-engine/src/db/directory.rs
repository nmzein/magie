use super::common::*;
use shared::{constants::BIN_ID, types::MoveMode};

#[wrap_with_store(insert)]
pub fn insert_<C>(conn: C, parent_id: u32, name: &str) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    let id = counter_(&conn)?;

    let mut stmt =
        conn.prepare_cached("INSERT INTO directories (id, name, parent_id) VALUES (?1, ?2, ?3);")?;
    stmt.execute((id, name, parent_id))?;

    Ok(())
}

#[wrap_with_store(delete)]
pub fn delete_<C>(conn: C, directory_id: u32) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    let mut stmt = conn.prepare_cached("DELETE FROM directories WHERE id = ?1;")?;
    stmt.execute([directory_id])?;

    Ok(())
}

#[wrap_with_store(r#move)]
pub fn r#move_<C>(conn: C, directory_id: u32, destination_id: u32, mode: &MoveMode) -> Result<()>
where
    C: Deref<Target = Connection>,
{
    match mode {
        MoveMode::Regular => {
            let mut stmt = conn.prepare_cached(
                "
                    UPDATE directories
                    SET parent_id = ?1
                    WHERE id = ?2;
                ",
            )?;
            stmt.execute((destination_id, directory_id))?;
        }
        MoveMode::SoftDelete => {
            let mut stmt = conn.prepare_cached(
                "
                    UPDATE directories
                    SET predeletion_parent_id = parent_id,
                        parent_id = ?1
                    WHERE id = ?2;
                ",
            )?;
            stmt.execute((BIN_ID, directory_id))?;
        }
    }

    Ok(())
}

#[wrap_with_store(is_within)]
pub fn is_within_<C>(conn: C, descendant_id: u32, ancestor_id: u32) -> Result<bool>
where
    C: Deref<Target = Connection>,
{
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
