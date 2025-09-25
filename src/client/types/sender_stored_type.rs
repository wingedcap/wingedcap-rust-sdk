use serde::{Deserialize, Serialize};

use super::{KeyWithMeta, SenderKeySet};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SenderStored {
    pub label: String,

    pub keys: Vec<KeyWithMeta>,

    pub sets: Vec<SenderKeySet>,
}
