use shared::functions::{declare_modules, find_modules};
use std::io::Result;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let mut common = File::create("src/common.rs")?;
    let mut export = File::create("src/export.rs")?;
    let mut lib = File::create("src/lib.rs")?;

    let encoders = find_modules();
    declare_modules(&mut lib, &encoders);
    declare_deps(&mut common)?;
    declare_exports(&mut export, &encoders)
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

fn declare_exports(export: &mut File, encoders: &Vec<String>) -> Result<()> {
    writeln!(
        export,
        "/// Auto-generated file. Any changes will be overwritten."
    )?;
    writeln!(export, "use crate::common::*;")?;

    if encoders.is_empty() {
        writeln!(
            export,
            r#"
pub fn get(_name: &str) -> Option<impl Encoder> {{ None }}

pub fn names() -> Vec<&'static str> {{ vec![] }}"#
        )?;

        return Ok(());
    }

    writeln!(
        export,
        r#"
pub fn get(name: &str) -> Option<impl Encoder> {{
    match name {{"#
    )?;

    for encoder in encoders.clone() {
        writeln!(
            export,
            "        crate::{encoder}::NAME => Some(crate::{encoder}::Module),"
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
        writeln!(export, "        crate::{encoder}::NAME,")?;
    }

    writeln!(
        export,
        r#"    ]
}}"#
    )?;

    Ok(())
}
