use crate::db::common::*;
use crate::types::{Directory, File};
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

pub struct Database {
    pub conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn init(path: &str, url: &str) -> Self {
        // Create the database file if it does not exist.
        if !Path::new(path).exists() {
            fs::File::create(path).unwrap();
        }

        let mut conn = Connection::open(url).unwrap();

        let migrations = Migrations::new(vec![M::up(include_str!("../../../state/schema.sql"))]);
        // Update the database schema atomically.
        migrations.to_latest(&mut conn).unwrap();

        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

/// Fetches all directories and subdirectories using a recursive CTE
pub fn get_registry() -> Result<Directory> {
    let conn = RDB.conn.lock().unwrap();

    let mut stmt = conn.prepare(
        r#"
            SELECT id, name, parent_id FROM images;
        "#,
    )?;
    let files = stmt.query_map([], |row| {
        Ok(File {
            r#type: "image".into(),
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;

    // Insert files into hashmap based on parent id.
    let mut filemap: HashMap<u32, Vec<File>> = HashMap::new();
    for file in files {
        let file = file?;
        let parent_id = file.parent_id;

        if let Some(files) = filemap.get_mut(&parent_id) {
            files.push(file);
        } else {
            filemap.insert(parent_id, vec![file]);
        }
    }

    // Empty registry stack.
    let mut registry: Vec<Directory> = Vec::new();

    // Retrieve all directories.
    let mut stmt = conn.prepare(
        r#"
            SELECT id, name, lft, rgt FROM directories ORDER BY lft ASC;
        "#,
    )?;
    let directories = stmt.query_map([], |row| {
        Ok(Directory {
            r#type: "directory".into(),
            id: row.get(0)?,
            name: row.get(1)?,
            lft: row.get(2)?,
            rgt: row.get(3)?,
            subdirectories: Vec::new(),
            files: Vec::new(),
        })
    })?;

    for directory in directories {
        let mut directory = directory?;

        // Check stack only if it contains elements.
        // If top element on the stack has a smaller right value than
        // the current directory, it means it is not a parent of the current directory.
        // Need to collapse the stack until the top element has a right value greater than the current directory.
        // TODO: Fix unwrap.
        while registry.len() > 1 && directory.rgt > registry.last().unwrap().rgt {
            collapse_stack(&mut registry);
        }

        // Get files of current directory before adding it to stack.
        if let Some(files) = filemap.get(&directory.id) {
            directory.files = files.clone();
        }

        // Push current directory onto the stack.
        registry.push(directory);
    }

    // Do final collapse.
    while registry.len() > 1 {
        collapse_stack(&mut registry);
    }

    Ok(registry[0].clone())
}

fn collapse_stack(registry: &mut Vec<Directory>) {
    // Add current top element of stack to subdirs of the one below it.
    if let Some(parent) = registry.pop() {
        if let Some(grandparent) = registry.last_mut() {
            grandparent.subdirectories.push(parent);
        }
    }
}
