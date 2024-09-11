use shared::functions::{declare_modules, find_modules};
use std::io::Result;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let mut common = File::create("src/common.rs")?;
    let mut export = File::create("src/export.rs")?;
    let mut lib = File::create("src/lib.rs")?;

    let encoders = find_modules();
    declare_modules(&mut lib, encoders.clone());

    if encoders.is_empty() {
        return handle_no_encoders(&mut export);
    }

    declare_deps(&mut common)?;
    handle_encoders(&mut export, encoders)
}

fn declare_deps(common: &mut File) -> Result<()> {
    writeln!(
        common,
        r#"pub use anyhow::Result;
pub use shared::{{
    constants::*,
    structs::{{Address, Region, Size, MetadataLayer}},
    traits::{{Decoder, Encoder}},
}};
pub use std::{{path::Path, sync::Arc}};
pub use zarrs::{{
    array::{{ArrayBuilder, DataType, FillValue}},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
}};"#
    )?;

    Ok(())
}

fn handle_no_encoders(export: &mut File) -> Result<()> {
    writeln!(
        export,
        r#"use shared::traits::Encoder;

pub fn get(_name: &str) -> Option<Box<dyn Encoder>> {{
    None
}}

pub const NAMES: [&str; 0] = [];"#
    )?;

    Ok(())
}

fn handle_encoders(export: &mut File, encoders: Vec<String>) -> Result<()> {
    writeln!(
        export,
        r#"use shared::traits::Encoder;

pub fn get(name: &str) -> Option<Box<dyn Encoder>> {{
    match name {{"#
    )?;

    for encoder in encoders.clone() {
        writeln!(
            export,
            r#"        crate::{}::NAME => Some(Box::new(crate::{}::Module)),"#,
            encoder, encoder
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

    for encoder in encoders {
        writeln!(export, r#"        crate::{}::NAME,"#, encoder,)?;
    }

    writeln!(
        export,
        r#"    ]
}}"#
    )?;

    Ok(())
}
