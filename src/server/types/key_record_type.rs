use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct KeyRecord {
    pub service: String,
    pub sender: String,
    pub receiver: String,
    pub key: String,
    pub timelock: u64,
    pub unlocks_at: u64,
}
