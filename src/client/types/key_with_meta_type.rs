use serde::{Deserialize, Serialize};

use crate::client::ServerMeta;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct KeyWithMeta {
    pub host: String,
    pub pk: String,
    pub id: String,
    pub meta: Option<ServerMeta>,
}
