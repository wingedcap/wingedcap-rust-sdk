use hex::FromHex;

use k256::ecdsa::SigningKey;

pub fn sk_to_pk(sk: &str) -> Result<String, String> {
    let sk_bytes = Vec::from_hex(sk).map_err(|e| format!("Invalid secret key: {}", e))?;

    let secret_key =
        SigningKey::from_slice(&sk_bytes).map_err(|e| format!("Invalid secret key: {}", e))?;

    let public_key = secret_key.verifying_key();

    let public_key_bytes = public_key.to_sec1_bytes();

    let public_key_hex = hex::encode(public_key_bytes);

    Ok(public_key_hex)
}
