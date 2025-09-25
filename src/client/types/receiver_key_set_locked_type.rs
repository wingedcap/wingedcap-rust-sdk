use serde::{Deserialize, Serialize};

use super::ReceiverKeyState;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ReceiverKeySetLocked {
    pub keys: Vec<ReceiverKeyState>,
    pub encrypted_data: String,
}
