use crate::api::{GET_KEY_ENDPOINT, GetKeyInput, GetKeyOutput};

use super::super::{types::Server, utils::fetch};

pub async fn get_key(server: &Server, payload: &GetKeyInput) -> Result<GetKeyOutput, String> {
    fetch(server, GET_KEY_ENDPOINT, payload).await
}
