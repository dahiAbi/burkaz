use std::ffi::{c_char, c_void};

use crate::term::BurkazTerm;

#[macro_export]
macro_rules! term_from_ptr {
    ($ptr:expr) => {
        unsafe { crate::term::BurkazTerm::from_raw($ptr as *mut _) }
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_term_int(field_id: u32, value: i64) -> *const c_void {
    let term = BurkazTerm::new(field_id, &value.into());
    Box::new(term).into_raw().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_term_text(
    field_id: u32,
    value_ptr: *const c_char,
    value_len: usize,
) -> *const c_void {
    let value = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(value_ptr.cast(), value_len))
    };
    let term = BurkazTerm::new(field_id, &value.into());
    Box::new(term).into_raw().cast()
}
