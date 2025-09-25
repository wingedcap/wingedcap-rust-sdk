use serde::{Deserialize, Serialize};

use super::KeyIndexArray;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ReceiverKeySet {
    pub keys: KeyIndexArray,
    pub data: String,
}
