use crate::db::prelude::*;

pub fn counter(dbm: &DatabaseManager, store_id: u32) -> Result<u32> {
    let conn = dbm.store(store_id)?;
    let mut stmt =
        conn.prepare_cached("UPDATE id_counter SET next_id = next_id + 1 RETURNING next_id;")?;
    let id = stmt.query_row([], |row| row.get::<_, i64>(0))?;
    Ok(u32::try_from(id)?)
}
