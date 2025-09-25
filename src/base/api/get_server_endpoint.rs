use serde::{Deserialize, Serialize};

use crate::{GenericEndpoint, client::ServerWithMeta};

const ID: &str = "server";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerInput {}

pub const GET_SERVER_ENDPOINT: GenericEndpoint<GetServerInput, ServerWithMeta> =
    GenericEndpoint::new(ID);
