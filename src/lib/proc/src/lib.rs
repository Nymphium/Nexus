#[no_mangle]
pub extern "C" fn __nx_exit(status: i64) {
    std::process::exit(status as i32);
}
