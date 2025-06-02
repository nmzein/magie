pub static RGB_CHANNELS: u32 = 3;
pub static TILE_SIZE: u32 = 1024;
pub static TILE_LENGTH: usize = (TILE_SIZE * TILE_SIZE) as usize;
pub static TILE_SPLIT_LENGTH: usize = (TILE_SIZE * TILE_SIZE * RGB_CHANNELS) as usize;
