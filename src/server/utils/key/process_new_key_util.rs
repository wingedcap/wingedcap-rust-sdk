use crate::{SetKeyOutput, sender_id_to_service_id};

use crate::utils::{get_current_unix_time, rand_hex_str};

use crate::server::KeyRecord;

pub fn process_new_key(timelock: u64) -> Result<(KeyRecord, SetKeyOutput), String> {
    let now = get_current_unix_time() as u64;

    let sender = rand_hex_str();
    let receiver = rand_hex_str();
    let key = rand_hex_str();

    let service = sender_id_to_service_id(&sender);

    let key_record = KeyRecord {
        service: service.clone(),
        sender: sender.clone(),
        receiver: receiver.clone(),
        key: key.clone(),
        timelock: timelock,
        unlocks_at: now + timelock,
    };

    let set_output = SetKeyOutput {
        sender: sender.clone(),
        receiver: receiver.clone(),
        key: key.clone(),
    };

    Ok((key_record, set_output))
}
