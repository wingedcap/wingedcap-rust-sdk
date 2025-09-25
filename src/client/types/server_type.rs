use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub pk: String,
}
