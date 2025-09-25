use crate::{aes_encrypt, client::combine_keys};

pub fn encrypt(message: &str, keys: Vec<&str>) -> Result<String, String> {
    let combined_key = combine_keys(keys)?;

    let key_bytes = hex::decode(combined_key.clone())
        .map_err(|e| format!("failed to decode key: {}", e.to_string()))?;

    let key_hex = hex::encode(key_bytes);

    let ciphertext = aes_encrypt(message, &key_hex)?;

    Ok(ciphertext)
}
