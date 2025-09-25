use hex::FromHex;

use k256::ecdsa::{Signature, SigningKey, signature::Signer};

pub fn sign(sk: &str, message: &str) -> Result<String, String> {
    let sk_bytes = Vec::from_hex(sk).map_err(|e| format!("Invalid secret key: {}", e))?;

    let secret_key =
        SigningKey::from_slice(&sk_bytes).map_err(|e| format!("Invalid secret key: {}", e))?;

    let sig: Signature = secret_key.sign(message.as_bytes());

    let sig_bytes = sig.to_bytes();

    let sig_hex = hex::encode(sig_bytes);

    Ok(sig_hex)
}
