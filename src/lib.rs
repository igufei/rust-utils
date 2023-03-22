use std::time::{UNIX_EPOCH, SystemTime};

pub fn timestamp() -> u64 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs() * 1000 + u64::from(duration.subsec_nanos()) / 1_000_000;
    timestamp
}
