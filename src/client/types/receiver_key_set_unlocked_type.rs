use serde::{Deserialize, Serialize};

use super::Key;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ReceiverKeySetUnlocked {
    pub keys: Vec<Key>,
    pub encrypted_data: String,
    pub decrypted_data: String,
}
