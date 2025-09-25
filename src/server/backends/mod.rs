#[cfg(not(target_arch = "wasm32"))]
mod axum;
#[cfg(not(target_arch = "wasm32"))]
pub use axum::*;
