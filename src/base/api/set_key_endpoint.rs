use serde::{Deserialize, Serialize};

use super::super::types::GenericEndpoint;

const ID: &str = "set";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetKeyInput {
    pub timelock: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetKeyOutput {
    pub sender: String,
    pub receiver: String,
    pub key: String,
}

pub const SET_KEY_ENDPOINT: GenericEndpoint<SetKeyInput, SetKeyOutput> = GenericEndpoint::new(ID);
