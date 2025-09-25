use aes_gcm::{Aes256Gcm, Key};

use crate::{aes_decrypt, client::combine_keys};

pub fn decrypt(ciphertext: &str, keys: Vec<&str>) -> Result<String, String> {
    let combined_key = combine_keys(keys)?;

    let key_bytes = hex::decode(combined_key.clone())
        .map_err(|e| format!("failed to decode key: {}", e.to_string()))?;

    let aes_key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let aes_key_hex = hex::encode(aes_key);

    let plaintext = aes_decrypt(ciphertext, &aes_key_hex)?;

    Ok(plaintext)
}
