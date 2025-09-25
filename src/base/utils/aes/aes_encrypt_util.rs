use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};

pub fn aes_encrypt(message: &str, key: &str) -> Result<String, String> {
    let key_bytes = hex::decode(key).map_err(|e| {
        let error = format!("failed to decode aes key: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    let aes_key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let nonce = Nonce::from_slice(&key_bytes[0..12]);

    let encryption_result = Aes256Gcm::new(aes_key).encrypt(nonce, message.as_bytes());

    match encryption_result {
        Ok(ciphertext_bytes) => {
            let ciphertext = hex::encode(ciphertext_bytes);

            Ok(ciphertext)
        }

        Err(e) => Err(e.to_string()),
    }
}
