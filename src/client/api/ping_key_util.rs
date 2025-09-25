use crate::api::{PING_KEY_ENDPOINT, PingKeyInput, PingKeyOutput};

use super::super::{types::Server, utils::fetch};

pub async fn ping_key(server: &Server, payload: &PingKeyInput) -> Result<PingKeyOutput, String> {
    fetch(server, PING_KEY_ENDPOINT, payload).await
}
