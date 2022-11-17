use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerType {
    Mastodon,
    Pleroma,
    Akkoma,
    Misskey,
    GoToSocial
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceMeta {
    pub address: String,
    pub servertype: ServerType,
    pub name: String,
    pub description: String,
    pub users: u32,
    pub favicon: String,
    pub invite_only: bool,
    pub last_indexed: u64
}