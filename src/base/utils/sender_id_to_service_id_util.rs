use crate::utils::sha_256;

pub fn sender_id_to_service_id(sender_id: &str) -> String {
    sha_256(sender_id)
}
