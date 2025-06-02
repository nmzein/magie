use crate::constants::REGISTRY_URL;
use anyhow::Result;
use rusqlite::{
    Connection,
    types::{FromSql, ToSql, ToSqlOutput, ValueRef},
};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone)]
pub struct DatabaseManager {
    pub registry: Arc<Mutex<Connection>>,
    pub stores: HashMap<u32, Store>,
}

impl DatabaseManager {
    pub fn connect() -> Result<Self> {
        let conn = Connection::open(REGISTRY_URL)?;

        conn.execute(
            r#"
                CREATE TABLE IF NOT EXISTS stores (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    type TEXT NOT NULL,
                    name TEXT UNIQUE NOT NULL,
                    path TEXT UNIQUE,
                    url TEXT UNIQUE
                );
            "#,
            (),
        )?;

        crate::db::stores::create(&conn, &Interface::Local, "Local")?;

        let stores = crate::db::registry::get_(&conn)?
            .into_iter()
            .map(|properties| {
                (
                    properties.id,
                    Store {
                        connection: Arc::new(Mutex::new(
                            // FIXME: Dont unwrap
                            Connection::open(&properties.url).unwrap(),
                        )),
                        properties,
                    },
                )
            })
            .collect();

        Ok(Self {
            registry: Arc::new(Mutex::new(conn)),
            stores,
        })
    }

    pub fn registry(&self) -> MutexGuard<'_, Connection> {
        self.registry.lock().unwrap()
    }

    pub fn store(&self, store_id: u32) -> Result<MutexGuard<'_, Connection>> {
        self.stores
            .get(&store_id)
            .map(|s| s.connection.lock().unwrap())
            .ok_or_else(|| anyhow::anyhow!("Requested store does not exist."))
    }

    pub fn store_properties(&self, store_id: u32) -> Result<&StoreProperties> {
        self.stores
            .get(&store_id)
            .map(|s| &s.properties)
            .ok_or_else(|| anyhow::anyhow!("Requested store does not exist."))
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: Arc<Mutex<Connection>>,
    pub properties: StoreProperties,
}

#[derive(Serialize, Debug, Clone)]
pub struct StoreProperties {
    pub id: u32,
    #[serde(skip)]
    pub r#type: Interface,
    pub name: String,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub url: String,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum Interface {
    Local,
    Http,
    Cloud,
}

impl ToSql for Interface {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            Interface::Local => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Local"))),
            Interface::Http => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Http"))),
            Interface::Cloud => Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Cloud"))),
        }
    }
}

impl FromSql for Interface {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let res = match value {
            ValueRef::Text(b"Local") => Interface::Local,
            ValueRef::Text(b"Http") => Interface::Http,
            ValueRef::Text(b"Cloud") => Interface::Cloud,
            _ => return Err(rusqlite::types::FromSqlError::InvalidType),
        };

        Ok(res)
    }
}
