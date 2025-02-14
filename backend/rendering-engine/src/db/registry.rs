use super::common::*;

pub fn get() -> Result<Vec<StoreProperties>> {
    get_(DB.registry())
}

pub fn get_<C>(conn: C) -> Result<Vec<StoreProperties>>
where
    C: Deref<Target = Connection>,
{
    let mut stmt = conn.prepare_cached("SELECT id, type, name, path FROM stores;")?;

    let res = stmt
        .query_map([], |row| {
            Ok(StoreProperties {
                id: row.get(0)?,
                r#type: row.get(1)?,
                name: row.get(2)?,
                path: PathBuf::from(row.get::<_, String>(3)?),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}
