use crate::{GetKeyOutput, GetKeyOutputUnlocked, server::KeyRecord};

use crate::utils::get_current_unix_time;

pub fn process_get_key(key_record: &KeyRecord) -> Result<GetKeyOutput, String> {
    let now = get_current_unix_time() as u64;

    let is_locked = key_record.unlocks_at > now;

    let output = if is_locked {
        GetKeyOutput::Locked
    } else {
        let key = key_record.key.clone();

        GetKeyOutput::Unlocked(GetKeyOutputUnlocked { key })
    };

    Ok(output)
}
