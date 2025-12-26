use axum::{body::Bytes, extract::ws::Message};
use std::convert::TryFrom;
use wincode::{SchemaWrite, SchemaRead, error::{WriteError, ReadError}};

/////////////////////////////////
// Client -> Server
/////////////////////////////////

#[derive(SchemaWrite, SchemaRead)]
#[wincode(tag_encoding = "u8")]
pub enum ClientMsg {
    Tile(TileClientMsg),
}

#[derive(SchemaWrite, SchemaRead)]
pub struct TileClientMsg {
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

impl TryFrom<Bytes> for ClientMsg {
    type Error = ReadError;
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        wincode::deserialize(&bytes)
    }
}

/////////////////////////////////
// Server -> Client
/////////////////////////////////

#[derive(SchemaWrite, SchemaRead)]
#[wincode(tag_encoding = "u8")]
pub enum GeneralServerMsg {
    Error(String),
    Directory(DirectoryServerMsg),
}

#[derive(SchemaWrite, SchemaRead)]
#[wincode(tag_encoding = "u8")]
pub enum DirectoryServerMsg {
    Create {
        store_id: u32,
        parent_id: u32,
        id: u32,
        name: String,
    },
    Delete {
        store_id: u32,
        id: u32,
    },
    Move {
        store_id: u32,
        id: u32,
        destination_id: u32,
    },
    Rename {
        store_id: u32,
        id: u32,
        name: String,
    },
}

#[derive(SchemaWrite, SchemaRead)]
#[wincode(tag_encoding = "u8")]
pub enum AssetServerMsg {
    Error(String),
    Tile(TileServerMsg),
}

#[derive(SchemaWrite, SchemaRead)]
pub struct TileServerMsg {
    pub level: u32,
    pub x: u32,
    pub y: u32,
    pub buffer: Vec<u8>,
}

impl TryFrom<GeneralServerMsg> for Message {
    type Error = WriteError;
    fn try_from(msg: GeneralServerMsg) -> Result<Self, Self::Error> {
        let bytes = wincode::serialize(&msg)?;
        Ok(Message::Binary(bytes.to_vec().into()))
    }
}

impl TryFrom<AssetServerMsg> for Message {
    type Error = WriteError;
    fn try_from(msg: AssetServerMsg) -> Result<Self, Self::Error> {
        let bytes = wincode::serialize(&msg)?;
        Ok(Message::Binary(bytes.to_vec().into()))
    }
}
