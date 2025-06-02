use super::common::*;
use crate::types::database::StoreProperties;

pub fn get(dbm: &DatabaseManager) -> Result<Vec<StoreProperties>> {
    let conn = dbm.registry();
    get_(conn)
}

pub fn get_<C>(conn: C) -> Result<Vec<StoreProperties>>
where
    C: std::ops::Deref<Target = Connection>,
{
    let mut stmt = conn.prepare_cached("SELECT id, type, name, path, url FROM stores;")?;

    let res = stmt
        .query_map([], |row| {
            Ok(StoreProperties {
                id: row.get(0)?,
                r#type: row.get(1)?,
                name: row.get(2)?,
                path: PathBuf::from(row.get::<_, String>(3)?),
                url: row.get::<_, String>(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}
