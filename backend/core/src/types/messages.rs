use axum::{body::Bytes, extract::ws::Message};
use bincode::{
    config::{BigEndian, Configuration, Fixint},
    error::DecodeError,
};

// TODO: Cleaner way to map incoming message tags so single source of truth.
const C_TILE_TAG: u8 = 0;

pub enum ClientMsg {
    Tile(TileClientMsg),
}

#[derive(bincode::Decode)]
pub struct TileClientMsg {
    pub store_id: u32,
    pub id: u32,
    pub level: u32,
    pub x: u32,
    pub y: u32,
}

impl TryFrom<Bytes> for ClientMsg {
    type Error = bincode::error::DecodeError;

    fn try_from(msg: Bytes) -> Result<Self, <Self as TryFrom<Bytes>>::Error> {
        let msg = msg.as_ref();

        // Extract and match on the tag.
        let tag = *msg
            .get(0)
            .ok_or_else(|| DecodeError::Other("Missing message tag"))?;

        let payload = &msg[1..];

        let result = match tag {
            C_TILE_TAG => ClientMsg::Tile(decode::<TileClientMsg>(payload)?),
            _ => return Err(DecodeError::Other("Invalid message.")),
        };

        Ok(result)
    }
}

/////////////////////////////////

const S_ERROR_TAG: u8 = 0;
const S_TILE_TAG: u8 = 1;
const S_DIRECTORY_TAG: u8 = 2;
const S_DIRECTORY_CREATE_TAG: u8 = 0;
const S_DIRECTORY_DELETE_TAG: u8 = 1;
const S_DIRECTORY_MOVE_TAG: u8 = 2;
const S_DIRECTORY_RENAME_TAG: u8 = 3;

pub enum ServerMsg {
    Error(String),
    Tile(TileServerMsg),
    Directory(DirectoryServerMsg),
}

#[derive(bincode::Encode)]
pub struct TileServerMsg {
    pub store_id: u32,
    pub id: u32,
    pub level: u32,
    pub x: u32,
    pub y: u32,
    pub buffer: Vec<u8>,
}

#[derive(bincode::Encode)]
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

impl TryInto<Message> for ServerMsg {
    type Error = bincode::error::EncodeError;

    fn try_into(self) -> Result<Message, <Self as TryInto<Message>>::Error> {
        let payload = match &self {
            ServerMsg::Error(msg) => encode(&(S_ERROR_TAG, msg))?,
            ServerMsg::Tile(msg) => encode(&(S_TILE_TAG, msg))?,
            ServerMsg::Directory(msg) => {
                let subtag = match msg {
                    DirectoryServerMsg::Create { .. } => S_DIRECTORY_CREATE_TAG,
                    DirectoryServerMsg::Delete { .. } => S_DIRECTORY_DELETE_TAG,
                    DirectoryServerMsg::Move { .. } => S_DIRECTORY_MOVE_TAG,
                    DirectoryServerMsg::Rename { .. } => S_DIRECTORY_RENAME_TAG,
                };
                encode(&(S_DIRECTORY_TAG, subtag, msg))?
            }
        };

        Ok(Message::Binary(payload.into()))
    }
}

static BINCODE_DECODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

static BINCODE_ENCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .with_no_limit();

fn decode<D: bincode::de::Decode<()>>(src: &[u8]) -> Result<D, bincode::error::DecodeError> {
    Ok(bincode::decode_from_slice::<D, _>(src, BINCODE_DECODE_CONFIG)?.0)
}

fn encode<E: bincode::enc::Encode>(val: E) -> Result<Vec<u8>, bincode::error::EncodeError> {
    Ok(bincode::encode_to_vec(val, BINCODE_ENCODE_CONFIG)?)
}
