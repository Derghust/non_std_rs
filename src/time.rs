use std::time::{SystemTime, UNIX_EPOCH};

const NANOSECONDS: u64 = 1_000_000_000;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    let seconds = duration.as_secs();
    let nanoseconds = (duration.subsec_nanos() as f64 / NANOSECONDS as f64) as u64;
    let timestamp = seconds * NANOSECONDS + nanoseconds;

    timestamp
}
