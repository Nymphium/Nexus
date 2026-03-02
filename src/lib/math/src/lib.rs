const NX_MATH_ERROR_I64: i64 = i64::MIN;

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

fn math_error_i64(message: impl AsRef<str>) -> i64 {
    eprintln!("math error: {}", message.as_ref());
    NX_MATH_ERROR_I64
}

#[no_mangle]
pub extern "C" fn __nx_abs_i64(val: i64) -> i64 {
    val.abs()
}

#[no_mangle]
pub extern "C" fn __nx_max_i64(a: i64, b: i64) -> i64 {
    a.max(b)
}

#[no_mangle]
pub extern "C" fn __nx_min_i64(a: i64, b: i64) -> i64 {
    a.min(b)
}

#[no_mangle]
pub extern "C" fn __nx_mod_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        return math_error_i64("mod: division by zero");
    }
    a % b
}

#[no_mangle]
pub extern "C" fn __nx_abs_float(val: f64) -> f64 {
    val.abs()
}

#[no_mangle]
pub extern "C" fn __nx_sqrt(val: f64) -> f64 {
    val.sqrt()
}

#[no_mangle]
pub extern "C" fn __nx_floor(val: f64) -> f64 {
    val.floor()
}

#[no_mangle]
pub extern "C" fn __nx_ceil(val: f64) -> f64 {
    val.ceil()
}

#[no_mangle]
pub extern "C" fn __nx_pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

#[no_mangle]
pub extern "C" fn __nx_i64_to_float(val: i64) -> f64 {
    val as f64
}

#[no_mangle]
pub extern "C" fn __nx_float_to_i64(val: f64) -> i64 {
    val as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_i64_zero_returns_explicit_error_value() {
        assert_eq!(__nx_mod_i64(10, 0), NX_MATH_ERROR_I64);
    }
}
