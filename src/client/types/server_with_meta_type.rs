use serde::{Deserialize, Serialize};

use crate::client::ServerMeta;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ServerWithMeta {
    pub host: String,
    pub pk: String,
    pub meta: Option<ServerMeta>,
}
