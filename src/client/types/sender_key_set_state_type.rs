use serde::{Deserialize, Serialize};

use super::Key;

use super::SenderKeyState;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SenderKeySetState {
    Locked(Vec<SenderKeyState>),
    Unlocked(Vec<Key>),
}
