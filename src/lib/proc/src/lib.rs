use nexus_wasm_alloc::checked_ptr_len;

#[cfg(not(feature = "no_alloc_export"))]
#[no_mangle]
pub extern "C" fn allocate(size: i32) -> i32 {
    nexus_wasm_alloc::allocate(size)
}

#[cfg(not(feature = "no_alloc_export"))]
#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: i32, size: i32) {
    nexus_wasm_alloc::deallocate(ptr, size);
}

#[no_mangle]
pub extern "C" fn __nx_exit(status: i64) {
    std::process::exit(status as i32);
}

#[no_mangle]
pub extern "C" fn __nx_get_env(key_ptr: i32, key_len: i32) -> i64 {
    let Some((offset, len)) = checked_ptr_len(key_ptr, key_len) else {
        return 0;
    };
    let bytes = unsafe { std::slice::from_raw_parts(offset as *const u8, len) };
    let key = String::from_utf8_lossy(bytes);
    match std::env::var(key.as_ref()) {
        Ok(val) => nexus_wasm_alloc::store_string_result(val),
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn __nx_set_env(
    key_ptr: i32,
    key_len: i32,
    value_ptr: i32,
    value_len: i32,
) {
    let Some((k_offset, k_len)) = checked_ptr_len(key_ptr, key_len) else {
        return;
    };
    let key_bytes = unsafe { std::slice::from_raw_parts(k_offset as *const u8, k_len) };
    let key = String::from_utf8_lossy(key_bytes);

    let Some((v_offset, v_len)) = checked_ptr_len(value_ptr, value_len) else {
        return;
    };
    let val_bytes = unsafe { std::slice::from_raw_parts(v_offset as *const u8, v_len) };
    let val = String::from_utf8_lossy(val_bytes);

    unsafe { std::env::set_var(key.as_ref(), val.as_ref()) };
}
