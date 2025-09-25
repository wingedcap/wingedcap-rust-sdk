use crate::api::{SET_KEY_ENDPOINT, SetKeyInput, SetKeyOutput};

use super::super::{types::Server, utils::fetch};

pub async fn set_key(server: &Server, payload: &SetKeyInput) -> Result<SetKeyOutput, String> {
    fetch(server, SET_KEY_ENDPOINT, payload).await
}
