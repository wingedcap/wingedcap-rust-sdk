#[cfg(target_arch = "wasm32")]
pub fn get_current_unix_time() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_current_unix_time() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
