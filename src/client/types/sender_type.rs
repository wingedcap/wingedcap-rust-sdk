use serde::{Deserialize, Serialize};

use super::{Key, SenderKeySet};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Sender {
    pub keys: Vec<Key>,

    pub sets: Vec<SenderKeySet>,
}
