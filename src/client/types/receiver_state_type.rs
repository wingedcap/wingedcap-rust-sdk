use serde::{Deserialize, Serialize};

use crate::client::{ReceiverKeySetLocked, ReceiverKeySetUnlocked};

use super::ReceiverKeyState;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ReceiverState {
    Locked {
        keys: Vec<ReceiverKeyState>,
        sets: Vec<ReceiverKeySetLocked>,
    },

    Unlocked {
        keys: Vec<ReceiverKeyState>,
        locked_sets: Vec<ReceiverKeySetLocked>,
        unlocked_sets: Vec<ReceiverKeySetUnlocked>,
    },
}
