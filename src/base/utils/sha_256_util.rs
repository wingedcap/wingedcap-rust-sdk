use sha2::{Digest, Sha256};

pub fn sha_256<T>(input: T) -> String
where
    T: Into<String>,
{
    let mut hasher = Sha256::new();

    hasher.update(input.into());

    hex::encode(hasher.finalize())
}
