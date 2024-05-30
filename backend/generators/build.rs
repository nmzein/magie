use shared::functions::{declare_modules, find_modules};
use std::io::Result;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let mut common = File::create("src/common.rs")?;
    let mut export = File::create("src/export.rs")?;
    let mut lib = File::create("src/lib.rs")?;

    let generators = find_modules();
    declare_modules(&mut lib, generators.clone());

    if generators.is_empty() {
        return handle_no_generators(&mut export);
    }

    declare_deps(&mut common)?;
    handle_generators(&mut export, generators)
}

fn declare_deps(common: &mut File) -> Result<()> {
    writeln!(
        common,
        r#"pub use anyhow::Result;
pub use shared::{{structs::AnnotationLayer, traits::Generator}};
pub use std::path::PathBuf;"#
    )?;

    Ok(())
}

fn handle_no_generators(export: &mut File) -> Result<()> {
    writeln!(
        export,
        r#"use shared::traits::Generator;

pub fn get(_name: &str) -> Option<Box<dyn Generator>> {{
    None
}}

pub const NAMES: [&str; 0] = [];"#
    )?;

    Ok(())
}

fn handle_generators(export: &mut File, generators: Vec<String>) -> Result<()> {
    writeln!(
        export,
        r#"use shared::traits::Generator;

pub fn get(name: &str) -> Option<Box<dyn Generator>> {{
    match name {{"#
    )?;

    for generator in generators.clone() {
        writeln!(
            export,
            r#"        crate::{}::NAME => Some(Box::new(crate::{}::Module)),"#,
            generator, generator
        )?;
    }

    writeln!(
        export,
        r#"        _ => None,
    }}
}}

pub fn names() -> Vec<&'static str> {{
    vec!["#
    )?;

    for generator in generators {
        writeln!(export, r#"        crate::{}::NAME,"#, generator,)?;
    }

    writeln!(
        export,
        r#"    ]
}}"#
    )?;

    Ok(())
}
