use rand::{self, RngCore};

pub fn rand_hex_str() -> String {
    let length = 32;

    let mut rng = rand::thread_rng();

    let mut bytes = vec![0; length];

    rng.try_fill_bytes(&mut bytes).unwrap();

    hex::encode(bytes)
}
