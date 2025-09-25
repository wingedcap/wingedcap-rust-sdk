use crate::client::{Key, SenderKeySet, SenderKeySetState, SenderKeyState, SenderState};

pub async fn process_sender_state(
    keys_state: Vec<SenderKeyState>,
    sets: Vec<SenderKeySet>,
) -> Result<SenderState, String> {
    let sets_state: Vec<SenderKeySetState> = sets
        .iter()
        .map(|sender_key_set| {
            let key_indices = sender_key_set;

            let key_set_keys_state: Vec<SenderKeyState> = key_indices
                .iter()
                .enumerate()
                .filter_map(|(_, key_index)| {
                    let key = keys_state.get(*key_index as usize);

                    key.cloned()
                })
                .collect();

            let key_set_unlocked_keys: Vec<Key> = key_set_keys_state
                .iter()
                .filter_map(|key_state| match key_state {
                    SenderKeyState::Unlocked(key) => Some(Key {
                        id: key.id.clone(),
                        host: key.host.clone(),
                        pk: key.pk.clone(),
                    }),
                    _ => None,
                })
                .collect();

            let key_set_locked_keys: Vec<Key> = key_set_keys_state
                .iter()
                .filter_map(|key_state| match key_state {
                    SenderKeyState::Locked(key) => Some(Key {
                        id: key.id.clone(),
                        host: key.host.clone(),
                        pk: key.pk.clone(),
                    }),
                    _ => None,
                })
                .collect();

            if key_set_locked_keys.is_empty() {
                SenderKeySetState::Unlocked(key_set_unlocked_keys)
            } else {
                SenderKeySetState::Locked(key_set_keys_state)
            }
        })
        .collect();

    let locked_sets: Vec<Vec<SenderKeyState>> = sets_state
        .iter()
        .filter_map(|key_set| match key_set {
            SenderKeySetState::Locked(keys) => Some(keys.clone()),
            _ => None,
        })
        .collect();

    let unlocked_sets: Vec<Vec<Key>> = sets_state
        .iter()
        .filter_map(|key_set| match key_set {
            SenderKeySetState::Unlocked(keys) => Some(
                keys.iter()
                    .map(|key| Key {
                        id: key.id.clone(),
                        host: key.host.clone(),
                        pk: key.pk.clone(),
                    })
                    .collect(),
            ),
            _ => None,
        })
        .collect();

    let sender_state = if unlocked_sets.is_empty() {
        SenderState::Locked {
            keys: keys_state,
            sets: locked_sets,
        }
    } else {
        SenderState::Unlocked {
            keys: keys_state,
            locked_sets: locked_sets,
            unlocked_sets: unlocked_sets,
        }
    };

    Ok(sender_state)
}
