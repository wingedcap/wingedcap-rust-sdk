use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Key {
    pub host: String,
    pub pk: String,
    pub id: String,
}
