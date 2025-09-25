use crate::{PingKeyOutput, server::KeyRecord};

use crate::utils::get_current_unix_time;

pub fn process_ping_key(key_record: &mut KeyRecord) -> Result<PingKeyOutput, String> {
    let now = get_current_unix_time() as u64;

    let is_locked = key_record.unlocks_at > now;

    let output = if is_locked {
        PingKeyOutput::Locked
    } else {
        PingKeyOutput::Unlocked
    };

    if is_locked {
        key_record.unlocks_at = now + key_record.timelock;
    }

    Ok(output)
}
