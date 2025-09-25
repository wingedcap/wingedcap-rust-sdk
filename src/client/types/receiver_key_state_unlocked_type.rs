use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ReceiverKeyStateUnlocked {
    pub host: String,
    pub pk: String,
    pub id: String,
    pub key: String,
}
