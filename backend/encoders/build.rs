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

pub fn interleave<'a>(channels: &[u8], tile: &'a mut Vec<u8>) -> &'a [u8] {{
    tile.clear();
    tile.reserve(TILE_SPLIT_LENGTH);

    let rs = &channels[..TILE_LENGTH];
    let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
    let bs = &channels[TILE_LENGTH * 2..];

    tile.extend(
        rs.iter()
            .zip(gs)
            .zip(bs)
            .flat_map(|((&r, &g), &b)| [r, g, b]),
    );

    tile
}}"#
    )?;

    Ok(())
}

fn handle_no_encoders(export: &mut File) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn convert(
    _name: &str,
    _input_path: &Path,
    _output_path: &Path,
    _decoder: Box<dyn Decoder>,
) -> Result<Vec<MetadataLayer>> {{
    Err(anyhow::anyhow!("No encoders available."))
}}

pub fn retrieve(_name: &str, _image_path: &Path, _level: u32, _x: u32, _y: u32) -> Result<Vec<u8>> {{
    Err(anyhow::anyhow!("No encoders available."))
}}

pub const NAMES: [&str; 0] = [];"#
    )?;

    Ok(())
}

fn handle_encoders(export: &mut File, encoders: Vec<String>) -> Result<()> {
    writeln!(
        export,
        r#"/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;

pub fn convert(
    name: &str,
    input_path: &Path,
    output_path: &Path,
    decoder: Box<dyn Decoder>,
) -> Result<Vec<MetadataLayer>> {{
    match name {{"#
    )?;

    for encoder in encoders.clone() {
        writeln!(
            export,
            r#"        crate::{}::NAME => crate::{}::Module::convert(input_path, output_path, decoder),"#,
            encoder, encoder
        )?;
    }

    writeln!(
        export,
        r#"        _ => Err(anyhow::anyhow!("Encoder not found.")),
    }}
}}"#
    )?;

    writeln!(
        export,
        r#"pub fn retrieve(name: &str, image_path: &Path, level: u32, x: u32, y: u32) -> Result<Vec<u8>> {{
    match name {{"#
    )?;

    for encoder in encoders.clone() {
        writeln!(
            export,
            r#"        crate::{}::NAME => crate::{}::Module::retrieve(image_path, level, x, y),"#,
            encoder, encoder
        )?;
    }

    writeln!(
        export,
        r#"        _ => Err(anyhow::anyhow!("Encoder not found.")),
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
