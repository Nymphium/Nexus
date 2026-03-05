use std::io::{self, BufRead, Write};

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
pub extern "C" fn __nx_print(ptr: i32, len: i32) {
    let Some((offset, len)) = nexus_wasm_alloc::checked_ptr_len(ptr, len) else {
        return;
    };
    let bytes = unsafe { std::slice::from_raw_parts(offset as *const u8, len) };
    let mut out = io::stdout();
    let _ = out.write_all(bytes);
    let _ = out.flush();
}

#[no_mangle]
pub extern "C" fn __nx_read_line() -> i64 {
    let stdin = io::stdin();
    let mut line = String::new();
    match stdin.lock().read_line(&mut line) {
        Ok(_) => {
            // Strip trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            nexus_wasm_alloc::store_string_result(line)
        }
        Err(_) => 0,
    }
}
