use serde::{Deserialize, Serialize};

use super::super::types::GenericEndpoint;

const ID: &str = "ping";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingKeyInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum PingKeyOutput {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked,
}

pub const PING_KEY_ENDPOINT: GenericEndpoint<PingKeyInput, PingKeyOutput> =
    GenericEndpoint::new(ID);
