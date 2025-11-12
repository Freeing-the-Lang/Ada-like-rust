use std::time::{SystemTime, UNIX_EPOCH};

pub fn runtime_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn runtime_id() -> String {
    format!("ADA-LIKE-RUST-{}", runtime_timestamp())
}
