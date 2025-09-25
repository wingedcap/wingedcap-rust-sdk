use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ServerMeta {
    pub provider: Option<String>,
    pub hoster: Option<String>,
    pub location: Option<String>,
}
