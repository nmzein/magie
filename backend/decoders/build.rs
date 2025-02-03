use shared::functions::{declare_modules, find_modules};
use std::fs;
use std::io::Result;
use std::{collections::HashMap, fs::File, io::Write};
use syn::{Expr, Item, Lit};

fn main() -> Result<()> {
    let mut common = File::create("src/common.rs")?;
    let mut export = File::create("src/export.rs")?;
    let mut lib = File::create("src/lib.rs")?;

    let decoders = find_modules();
    declare_modules(&mut lib, decoders.clone());

    if decoders.is_empty() {
        return handle_no_decoders(&mut export);
    }

    declare_deps(&mut common)?;
    handle_decoders(&mut export, decoders)
}

fn declare_deps(common: &mut File) -> Result<()> {
    writeln!(
        common,
        r#"/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use image::{{ImageBuffer, Rgb}};
pub use shared::{{
    structs::{{Region, Size}},
    traits::Decoder,
}};
pub use std::path::Path;"#
    )?;

    Ok(())
}

fn handle_no_decoders(export: &mut File) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn get(_extension: &str) -> Vec<impl Decoder> {{
    vec![]
}}"#
    )?;

    Ok(())
}

fn handle_decoders(export: &mut File, decoders: Vec<String>) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
        
pub fn get(extension: &str) -> Vec<impl Decoder> {{
    match extension {{"#
    )?;

    // Loop over decoders, query them for their supported extensions
    // and populate extension map with decoder.
    let extension_map = create_extension_map(decoders.clone());
    // Sort map by extension in alphabetic order.
    let mut extension_map: Vec<_> = extension_map.into_iter().collect();
    extension_map.sort_by(|a, b| a.0.cmp(&b.0));

    // Loop over extension map.
    for (extension, decoders) in extension_map {
        writeln!(export, r#"        "{}" => vec!["#, extension)?;
        for decoder in decoders {
            writeln!(export, r#"            crate::{decoder}::Module,"#)?;
        }
        writeln!(export, r#"        ],"#)?;
    }

    writeln!(
        export,
        r#"        _ => vec![],
    }}
}}
"#
    )?;

    writeln!(
        export,
        r#"pub fn names(name: &str) -> Option<impl Decoder> {{
    match name {{"#
    )?;

    // Loop over decoders.
    for decoder in decoders {
        writeln!(
            export,
            r#"        crate::{decoder}::NAME => Some(crate::{decoder}::Module),"#,
        )?;
    }

    writeln!(
        export,
        r#"        _ => None,
    }}
}}"#
    )?;
    Ok(())
}

fn create_extension_map(decoders: Vec<String>) -> HashMap<String, Vec<String>> {
    // Initialize a HashMap to store unique extensions
    let mut extensions_map: HashMap<String, Vec<String>> = HashMap::new();

    for decoder in decoders {
        // Read the contents of the Rust file
        let contents =
            fs::read_to_string(&format!("src/{decoder}.rs")).expect("Failed to read file");

        // Parse the Rust code into a syntax tree
        let parsed: Vec<Item> = syn::parse_file(&contents).unwrap().items;

        // Find the constant named "EXTENSIONS"
        let extensions_const = parsed.iter().find_map(|item| {
            // Check if the item is a constant.
            if let Item::Const(item_const) = item {
                // Check if the constant is named "EXTENSIONS"
                if item_const.ident == "EXTENSIONS" {
                    Some(item_const)
                } else {
                    None
                }
            } else {
                None
            }
        });

        let mut extensions = Vec::new();
        // Extract the values from the "EXTENSIONS" constant
        if let Some(extensions_const) = extensions_const {
            // Check if the expression inside the constant is an array
            if let syn::Expr::Array(expr_array) = *extensions_const.expr.clone() {
                // Iterate over each element in the array
                for expr in expr_array.elems {
                    // Check if the element is a literal
                    if let Expr::Lit(lit) = expr {
                        // Check if the literal is a string
                        if let Lit::Str(lit_str) = lit.lit {
                            // If it is a string, extract its value and add it to the vector
                            extensions.push(lit_str.value());
                        }
                    }
                }
            }
            // Print the extracted values
            println!("{:?}", extensions);
        } else {
            println!("Error: Constant EXTENSIONS not found");
        }

        extensions.iter().for_each(|ext| {
            extensions_map
                .entry(ext.to_string())
                .or_default()
                .push(decoder.clone());
        });
    }

    extensions_map
}
