use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};

pub fn aes_decrypt(ciphertext: &str, key: &str) -> Result<String, String> {
    let key_bytes = hex::decode(key).map_err(|e| {
        let error = format!("failed to decode aes key: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    let aes_key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let nonce = Nonce::from_slice(&key_bytes[0..12]);

    let ciphertext_bytes = match hex::decode(ciphertext) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("Failed to decode hex ciphertext: {}", e)),
    };

    let decryption_result = Aes256Gcm::new(aes_key).decrypt(nonce, ciphertext_bytes.as_slice());

    match decryption_result {
        Ok(plaintext_bytes) => {
            let plaintext = match String::from_utf8(plaintext_bytes) {
                Ok(text) => text,
                Err(e) => {
                    return Err(format!(
                        "Failed to convert decrypted bytes to string: {}",
                        e
                    ));
                }
            };

            Ok(plaintext)
        }

        Err(e) => {
            tracing::error!("error decrypting: {:?}", e);
            Err(e.to_string())
        }
    }
}
