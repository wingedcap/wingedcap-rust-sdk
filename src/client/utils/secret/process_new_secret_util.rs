use crate::{
    SetKeyOutput,
    client::{Key, KeyIndexArray, Receiver, ReceiverKeySet, Sender, Server, encrypt},
};

pub async fn process_new_secret(
    message: String,
    keys: Vec<(Server, SetKeyOutput)>,
    sets: Vec<KeyIndexArray>,
) -> Result<(Sender, Receiver), String> {
    let sender = Sender {
        keys: keys
            .iter()
            .map(|(Server { host, pk }, SetKeyOutput { sender, .. })| Key {
                host: host.clone(),
                pk: pk.clone(),
                id: sender.clone(),
            })
            .collect(),

        sets: sets.iter().map(|set| set.clone()).collect(),
    };

    let receiver_keys = keys
        .iter()
        .map(|(Server { host, pk }, SetKeyOutput { receiver, .. })| Key {
            host: host.clone(),
            pk: pk.clone(),
            id: receiver.clone(),
        })
        .collect();

    let receiver_sets_results: Vec<Result<ReceiverKeySet, String>> = sets
        .iter()
        .map(|set| {
            let keys_indices = set;

            let key_materials_results: Vec<Result<&str, String>> = keys_indices
                .iter()
                .map(|key_index| {
                    let key = keys.get(*key_index as usize);

                    match key {
                        Some((_, SetKeyOutput { key, .. })) => Ok(key.as_str()),
                        None => Err(format!("key not found: {}", key_index)),
                    }
                })
                .collect();

            let key_materials: Vec<&str> = key_materials_results
                .iter()
                .filter_map(|result| result.clone().ok())
                .collect();

            let data = encrypt(&message, key_materials)?;

            Ok(ReceiverKeySet {
                keys: set.clone(),
                data: data.to_string(),
            })
        })
        .collect();

    let receiver_key_sets_ok: Vec<ReceiverKeySet> = receiver_sets_results
        .iter()
        .filter_map(|result| result.clone().ok())
        .collect();

    let receiver_key_sets_error: Vec<String> = receiver_sets_results
        .iter()
        .filter_map(|result| result.clone().err())
        .collect();

    if !receiver_key_sets_error.is_empty() {
        return Err(format!(
            "failed to set secret: {}",
            receiver_key_sets_error.join(", ")
        ));
    }

    let receiver_sets = receiver_key_sets_ok;

    let receiver = Receiver {
        keys: receiver_keys,
        sets: receiver_sets,
    };

    Ok((sender, receiver))
}
