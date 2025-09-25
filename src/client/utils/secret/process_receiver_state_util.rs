use crate::client::{
    types::{
        Key, ReceiverKeySet, ReceiverKeySetLocked, ReceiverKeySetState, ReceiverKeySetUnlocked,
        ReceiverKeyState, ReceiverKeyStateUnlocked, ReceiverState,
    },
    utils::decrypt,
};

pub async fn process_receiver_state(
    keys_state: Vec<ReceiverKeyState>,
    sets: Vec<ReceiverKeySet>,
) -> Result<ReceiverState, String> {
    let sets_state: Vec<ReceiverKeySetState> = sets
        .iter()
        .map(|key_set| {
            let ReceiverKeySet {
                keys: key_indices,
                data: key_set_encrypted_data,
            } = key_set;

            let key_set_keys_state: Vec<ReceiverKeyState> = key_indices
                .iter()
                .enumerate()
                .filter_map(|(_, key_index)| {
                    let key = keys_state.get(*key_index as usize);

                    match key {
                        Some(ReceiverKeyState::Locked(key)) => {
                            Some(ReceiverKeyState::Locked(key.clone()))
                        }

                        Some(ReceiverKeyState::Unlocked(key)) => {
                            Some(ReceiverKeyState::Unlocked(key.clone()))
                        }

                        None => None,
                    }
                })
                .collect();

            let key_set_unlocked_keys: Vec<ReceiverKeyStateUnlocked> = key_set_keys_state
                .iter()
                .filter_map(|key_state| match key_state {
                    ReceiverKeyState::Unlocked(key) => Some(ReceiverKeyStateUnlocked {
                        id: key.id.clone(),
                        host: key.host.clone(),
                        pk: key.pk.clone(),
                        key: key.key.clone(),
                    }),
                    _ => None,
                })
                .collect();

            let key_set_locked_keys: Vec<Key> = key_set_keys_state
                .iter()
                .filter_map(|key_state| match key_state {
                    ReceiverKeyState::Locked(key) => Some(Key {
                        id: key.id.clone(),
                        host: key.host.clone(),
                        pk: key.pk.clone(),
                    }),
                    _ => None,
                })
                .collect();

            if key_set_locked_keys.is_empty() {
                let decryption_result = {
                    let keys_material: Vec<&str> = key_set_unlocked_keys
                        .iter()
                        .map(|key| key.key.as_str())
                        .collect();

                    decrypt(key_set_encrypted_data, keys_material)
                };

                match decryption_result {
                    Ok(decrypted_data) => ReceiverKeySetState::Unlocked {
                        keys: key_set_unlocked_keys
                            .iter()
                            .map(|key| Key {
                                id: key.id.clone(),
                                host: key.host.clone(),
                                pk: key.pk.clone(),
                            })
                            .collect(),

                        encrypted_data: key_set_encrypted_data.clone(),

                        decrypted_data,
                    },

                    Err(_) => {
                        tracing::error!("failed to decrypt key set");

                        ReceiverKeySetState::Locked {
                            keys: key_set_keys_state,
                            encrypted_data: key_set_encrypted_data.clone(),
                        }
                    }
                }
            } else {
                ReceiverKeySetState::Locked {
                    keys: key_set_keys_state,
                    encrypted_data: key_set_encrypted_data.clone(),
                }
            }
        })
        .collect();

    let locked_sets: Vec<ReceiverKeySetLocked> = sets_state
        .iter()
        .filter_map(|key_set| match key_set {
            ReceiverKeySetState::Locked {
                keys,
                encrypted_data,
            } => Some(ReceiverKeySetLocked {
                keys: keys.clone(),
                encrypted_data: encrypted_data.clone(),
            }),
            _ => None,
        })
        .collect();

    let unlocked_sets: Vec<ReceiverKeySetUnlocked> = sets_state
        .iter()
        .filter_map(|key_set| match key_set {
            ReceiverKeySetState::Unlocked {
                keys,
                encrypted_data,
                decrypted_data,
            } => Some(ReceiverKeySetUnlocked {
                keys: keys.clone(),
                encrypted_data: encrypted_data.clone(),
                decrypted_data: decrypted_data.clone(),
            }),
            _ => None,
        })
        .collect();

    let receiver_state = if unlocked_sets.is_empty() {
        ReceiverState::Locked {
            keys: keys_state,
            sets: locked_sets,
        }
    } else {
        ReceiverState::Unlocked {
            keys: keys_state,
            locked_sets: locked_sets,
            unlocked_sets: unlocked_sets,
        }
    };

    Ok(receiver_state)
}
