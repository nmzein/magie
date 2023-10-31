use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

pub fn parse<T: for <'a> Deserialize<'a>>(file_path: &PathBuf) -> anyhow::Result<T> {
    // Parses JSON file and dumps data into a String.
    let raw_data = fs::read_to_string(file_path)?;

    // Deserializes JSON string into a struct of type T.
    let deserialized_data: T = serde_json::from_str(&raw_data)?;

    Ok(deserialized_data)
}