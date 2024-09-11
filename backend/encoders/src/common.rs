/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use shared::{
    constants::*,
    structs::{Address, MetadataLayer, Region, Size},
    traits::{Decoder, Encoder},
};
pub use std::{path::Path, sync::Arc};
pub use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset,
    group::GroupBuilder,
    storage::store::FilesystemStore,
};

pub fn interleave<'a>(channels: &[u8], tile: &'a mut Vec<u8>) -> &'a [u8] {
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
}
