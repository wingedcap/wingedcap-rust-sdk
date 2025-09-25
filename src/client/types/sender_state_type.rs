use serde::{Deserialize, Serialize};

use super::SenderKeyState;

use super::Key;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SenderState {
    Locked {
        keys: Vec<SenderKeyState>,
        sets: Vec<Vec<SenderKeyState>>,
    },

    Unlocked {
        keys: Vec<SenderKeyState>,
        locked_sets: Vec<Vec<SenderKeyState>>,
        unlocked_sets: Vec<Vec<Key>>,
    },
}
