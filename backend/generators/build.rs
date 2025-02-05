use shared::functions::{declare_modules, find_modules};
use std::io::Result;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let mut common = File::create("src/common.rs")?;
    let mut export = File::create("src/export.rs")?;
    let mut lib = File::create("src/lib.rs")?;

    let generators = find_modules();
    declare_modules(&mut lib, &generators);
    declare_deps(&mut common)?;
    declare_exports(&mut export, &generators)
}

fn declare_deps(common: &mut File) -> Result<()> {
    writeln!(
        common,
        r#"/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use shared::{{structs::AnnotationLayer, traits::Generator}};
pub use std::path::Path;"#
    )?;

    Ok(())
}

fn declare_exports(export: &mut File, generators: &Vec<String>) -> Result<()> {
    writeln!(
        export,
        "/// Auto-generated file. Any changes will be overwritten."
    )?;
    writeln!(export, "use crate::common::*;")?;

    if generators.is_empty() {
        writeln!(
            export,
            r#"
pub fn get(_name: &str) -> Option<impl Generator> {{ None }}

pub fn names() -> Vec<&'static str> {{ vec![] }}"#
        )?;

        return Ok(());
    }

    writeln!(
        export,
        r#"
pub fn get(name: &str) -> Option<impl Generator> {{
    match name {{"#
    )?;

    for generator in generators.clone() {
        writeln!(
            export,
            "        crate::{generator}::NAME => Some(crate::{generator}::Module),"
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
        writeln!(export, "        crate::{generator}::NAME,")?;
    }

    writeln!(
        export,
        r#"    ]
}}"#
    )?;

    Ok(())
}
