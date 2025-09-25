use serde::{Deserialize, Serialize};

use super::Key;

use super::ReceiverKeyState;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ReceiverKeySetState {
    Locked {
        keys: Vec<ReceiverKeyState>,
        encrypted_data: String,
    },

    Unlocked {
        keys: Vec<Key>,
        encrypted_data: String,
        decrypted_data: String,
    },
}
