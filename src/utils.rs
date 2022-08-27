use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_ts() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time error")
        .as_millis()
}
