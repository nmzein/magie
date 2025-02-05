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
        r#"/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use shared::{{
    constants::*,
    structs::{{Address, MetadataLayer, Region, Size}},
    traits::{{Decoder, Encoder}},
}};
pub use std::{{path::Path, sync::Arc}};
pub use zarrs::{{
    array::{{codec::GzipCodec, Array, ArrayBuilder, DataType, FillValue}},
    array_subset::ArraySubset,
    filesystem::FilesystemStore,
    group::GroupBuilder,
}};

pub fn interleave(channels: &[u8], output: &mut Vec<u8>) {{
    let rs = &channels[..TILE_LENGTH];
    let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
    let bs = &channels[TILE_LENGTH * 2..];

    output.extend(
        rs.iter()
            .zip(gs)
            .zip(bs)
            .flat_map(|((&r, &g), &b)| [r, g, b]),
    );
}}"#
    )?;

    Ok(())
}

fn handle_no_encoders(export: &mut File) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn get(_name: &str) -> Option<impl Encoder> {{ None }}

pub const NAMES: [&str; 0] = [];"#
    )?;

    Ok(())
}

fn handle_encoders(export: &mut File, encoders: Vec<String>) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn get(name: &str) -> Option<impl Encoder> {{
    match name {{"#
    )?;

    for encoder in encoders.clone() {
        writeln!(
            export,
            r#"        crate::{}::NAME => Some(crate::{}::Module),"#,
            encoder, encoder
        )?;
    }

    writeln!(
        export,
        r#"        _ => None,
    }}
}}
"#
    )?;

    writeln!(
        export,
        r#"pub fn names() -> Vec<&'static str> {{
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
