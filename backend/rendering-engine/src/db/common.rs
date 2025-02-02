pub use crate::RDB;
pub use anyhow::Result;
pub use rusqlite::Connection;
#[cfg(feature = "log.database")]
use std::fmt::Debug;

#[cfg(feature = "log.database")]
pub fn log<T: Debug>(operation: &str, result: Option<&T>) {
    print!("Database <{}>", operation);
    if let Some(result) = result {
        print!(": {:#?}", result);
    }
    println!("\n");
}
