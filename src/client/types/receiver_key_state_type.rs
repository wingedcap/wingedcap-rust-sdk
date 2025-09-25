use serde::{Deserialize, Serialize};

use super::{Key, ReceiverKeyStateUnlocked};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ReceiverKeyState {
    Locked(Key),
    Unlocked(ReceiverKeyStateUnlocked),
}
