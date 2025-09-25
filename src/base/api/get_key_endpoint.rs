use serde::{Deserialize, Serialize};

use super::super::types::GenericEndpoint;

const ID: &str = "get";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKeyInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKeyOutputUnlocked {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum GetKeyOutput {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked(GetKeyOutputUnlocked),
}

pub const GET_KEY_ENDPOINT: GenericEndpoint<GetKeyInput, GetKeyOutput> = GenericEndpoint::new(ID);
