use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[no_mangle]
pub extern "C" fn __nx_sleep(ms: i64) {
    if ms > 0 {
        thread::sleep(Duration::from_millis(ms as u64));
    }
}

#[no_mangle]
pub extern "C" fn __nx_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
