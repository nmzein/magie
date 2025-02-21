/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use shared::{
    constants::*,
    traits::{Decoder, Encoder},
    types::{Address, MetadataLayer, Region, Size},
};
pub use std::{path::Path, sync::Arc};
pub use zarrs::{
    array::{Array, ArrayBuilder, DataType, FillValue, codec::GzipCodec},
    array_subset::ArraySubset,
    filesystem::FilesystemStore,
    group::GroupBuilder,
};
pub fn interleave(channels: &[u8], output: &mut Box<[u8]>) {
    let rs = &channels[..TILE_LENGTH];
    let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
    let bs = &channels[TILE_LENGTH * 2..];
    for (i, ((&r, &g), &b)) in rs.iter().zip(gs).zip(bs).enumerate() {
        let idx = i * 3;
        output[idx] = r;
        output[idx + 1] = g;
        output[idx + 2] = b;
    }
}
