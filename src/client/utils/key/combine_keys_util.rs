pub fn combine_keys(keys: Vec<&str>) -> Result<String, String> {
    let key_length = 32;

    let mut combined_key_bytes = vec![0u8; key_length];

    for key in keys {
        let key_bytes =
            hex::decode(key).map_err(|e| format!("failed to decode key: {}", e.to_string()))?;

        for i in 0..key_length {
            match key_bytes.get(i) {
                Some(key_byte) => {
                    combined_key_bytes[i] = combined_key_bytes[i].wrapping_add(*key_byte)
                }

                None => break,
            }
        }
    }

    let combined_key_hex = hex::encode(combined_key_bytes);

    Ok(combined_key_hex)
}
