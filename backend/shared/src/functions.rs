use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
};

pub fn find_modules() -> Vec<String> {
    let mut modules = Vec::new();

    let enabled: Vec<String> = env::vars()
        .filter_map(|(key, _)| {
            if key.starts_with("CARGO_FEATURE_") {
                Some(key.trim_start_matches("CARGO_FEATURE_").to_string())
            } else {
                None
            }
        })
        .collect();

    let entries = fs::read_dir("src").expect("Failed to read src directory");
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            // Filter for files with the .rs extension.
            let extension = path.extension();
            if extension.is_none() || extension.unwrap() != "rs" {
                continue;
            }
            if let Some(filename) = path.file_stem() {
                // Filter out unwanted files.
                if filename == "common" || filename == "export" || filename == "lib" {
                    continue;
                }
                let module_name = filename.to_string_lossy().to_string();
                if enabled.contains(&module_name.to_uppercase().replace("-", "_")) {
                    modules.push(module_name);
                }
            }
        }
    }

    modules
}

pub fn find_exported_struct(contents: File) -> Option<String> {
    // Read the file contents
    let contents = io::BufReader::new(contents);
    // Iterate over each line in the file
    for line in contents.lines() {
        // Check if the line contains a public struct declaration
        if let Some(captures) = line.unwrap().strip_prefix("pub struct ") {
            // Extract the struct name
            if let Some(struct_name) = captures.split(';').next() {
                return Some(struct_name.trim().to_string());
            }
        }
    }
    None
}

pub fn declare_modules(mut file: &mut dyn Write, modules: Vec<String>) {
    writeln!(
        &mut file,
        r#"mod common;
pub mod export;
"#
    )
    .expect("Could not write module declaration to file.");

    for module in modules {
        writeln!(&mut file, "#[cfg(feature = \"{}\")]", module)
            .expect("Could not write module declaration to file.");
        writeln!(&mut file, "mod {};", module)
            .expect("Could not write module declaration to file.");
    }
}
