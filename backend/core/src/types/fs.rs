use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq)]
pub enum DeleteMode {
    #[serde(alias = "hard")]
    Hard,
    #[serde(alias = "soft")]
    Soft,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Entry {
    Directory(Directory),
    Asset(Asset),
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}
