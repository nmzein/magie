pub use anyhow::Result;
pub use macros::wrap_with_store;
pub use rusqlite::Connection;
pub use serde::Serialize;
pub use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use rusqlite::types::{FromSql, ToSql, ToSqlOutput, ValueRef};
use rusqlite_migration::{M, Migrations};
use std::sync::MutexGuard;
use std::{collections::HashMap, sync::LazyLock};
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

pub static REGISTRY_PATH: &str = "../databases/registry.sqlite";
pub static REGISTRY_URL: &str = "sqlite://../../databases/registry.sqlite";
pub static STORES_DATABASE_PATH_PREFIX: &str = "../databases/s";
pub static STORES_DATABASE_URL_PREFIX: &str = "sqlite://../../databases/s";

pub static DB: LazyLock<Databases> = LazyLock::new(Databases::connect);

pub struct Databases {
    pub registry: Arc<Mutex<Connection>>,
    pub stores: HashMap<u32, Store>,
}

// TODO: Init stores using select on registry.
impl Databases {
    pub fn connect() -> Self {
        // Create the database file if it does not exist.
        if !Path::new(REGISTRY_PATH).exists() {
            fs::File::create(REGISTRY_PATH).unwrap();
        }

        let mut conn = Connection::open(REGISTRY_URL).unwrap();

        let migrations = Migrations::new(vec![M::up(include_str!(
            "../../../../databases/registry.sql"
        ))]);
        // Update the database schema atomically.
        migrations.to_latest(&mut conn).unwrap();

        crate::db::stores::create_(
            &conn,
            &DatabaseType::Local,
            "Local",
            &PathBuf::from("../stores"),
        )
        .unwrap();

        let stores = super::registry::get_(&conn)
            .unwrap()
            .into_iter()
            .map(|properties| {
                (
                    properties.id,
                    Store {
                        connection: Arc::new(Mutex::new(
                            Connection::open(format!(
                                "{STORES_DATABASE_URL_PREFIX}{}.sqlite",
                                properties.id
                            ))
                            .unwrap(),
                        )),
                        properties,
                    },
                )
            })
            .collect();

        Self {
            registry: Arc::new(Mutex::new(conn)),
            stores,
        }
    }

    pub fn registry(&self) -> MutexGuard<'_, Connection> {
        self.registry.lock().unwrap()
    }

    pub fn store(&self, store_id: u32) -> Option<Arc<Mutex<Connection>>> {
        self.stores
            .get(&store_id)
            .map(|s| Arc::clone(&s.connection))
    }
}

#[derive(Debug)]
pub struct Store {
    pub connection: Arc<Mutex<Connection>>,
    pub properties: StoreProperties,
}

#[derive(Serialize, Debug)]
pub struct StoreProperties {
    pub id: u32,
    #[serde(skip)]
    pub r#type: DatabaseType,
    pub name: String,
    #[serde(skip)]
    pub path: PathBuf,
}

impl StoreProperties {
    pub fn bin(&self) -> PathBuf {
        self.path.join("Bin")
    }

    pub fn image(&self, image_id: u32) -> PathBuf {
        self.path.join(format!("i{image_id}"))
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub enum DatabaseType {
    Local,
    Http,
    Cloud,
}

impl ToSql for DatabaseType {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            DatabaseType::Local => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Local"))),
            DatabaseType::Http => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Http"))),
            DatabaseType::Cloud => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Cloud"))),
        }
    }
}

impl FromSql for DatabaseType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let res = match value {
            ValueRef::Text(b"Local") => DatabaseType::Local,
            ValueRef::Text(b"Http") => DatabaseType::Http,
            ValueRef::Text(b"Cloud") => DatabaseType::Cloud,
            _ => return Err(rusqlite::types::FromSqlError::InvalidType),
        };

        Ok(res)
    }
}

pub fn counter(store_id: u32) -> Result<u32> {
    let conn = DB.store(store_id);
    if let Some(conn) = conn {
        return counter_(&conn.lock().unwrap());
    };
    Err(anyhow::anyhow!("Store not found"))
}

pub fn counter_(conn: &Connection) -> Result<u32> {
    let mut stmt =
        conn.prepare_cached("UPDATE id_counter SET next_id = next_id + 1 RETURNING next_id;")?;
    let id = stmt.query_row([], |row| row.get::<_, i64>(0))?;
    Ok(u32::try_from(id)?)
}
