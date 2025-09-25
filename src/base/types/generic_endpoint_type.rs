use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct GenericEndpoint<I, O> {
    pub id: &'static str,
    _request: PhantomData<I>,
    _response: PhantomData<O>,
}

impl<I, O> GenericEndpoint<I, O> {
    /// Creates a new GenericEndpoint with the given id.
    /// PhantomData fields are automatically initialized.
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            _request: PhantomData,
            _response: PhantomData,
        }
    }
}
