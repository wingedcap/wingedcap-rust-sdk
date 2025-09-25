use serde::{Deserialize, Serialize};

use super::{Key, ReceiverKeySet};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Receiver {
    pub keys: Vec<Key>,

    pub sets: Vec<ReceiverKeySet>,
}
