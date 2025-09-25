use std::str::FromStr;

use hex::FromHex;

use k256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

pub fn verify(pk: &str, message: &str, signature: &str) -> Result<bool, String> {
    let pk_bytes = Vec::from_hex(pk).map_err(|e| {
        let error = format!("Invalid public key: {}", e);
        tracing::error!(error);
        error
    })?;

    let public_key = VerifyingKey::from_sec1_bytes(&pk_bytes).map_err(|e| {
        let error = format!("Invalid public key: {}", e);
        tracing::error!(error);
        error
    })?;

    let sig = Signature::from_str(signature).map_err(|e| format!("Invalid signature: {}", e))?;

    let sig = public_key.verify(message.as_bytes(), &sig);

    let is_valid = sig.is_ok();

    Ok(is_valid)
}
