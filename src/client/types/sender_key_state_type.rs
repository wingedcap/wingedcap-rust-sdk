use serde::{Deserialize, Serialize};

use super::Key;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SenderKeyState {
    Locked(Key),
    Unlocked(Key),
}
