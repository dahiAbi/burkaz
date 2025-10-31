use std::ffi::{CStr, CString, c_char, c_void};

use crate::object::BurkazObject;

// #[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_new(data_arr_ptr: *mut u8, data_arr_len: usize) -> *const c_void {
    let data = unsafe { std::slice::from_raw_parts(data_arr_ptr, data_arr_len) };

    match BurkazObject::from_bytes(data) {
        Some(object) => object.into_raw().cast(),
        None => std::ptr::null_mut(),
    }
}

// #[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_bytes(
    object_ptr: *const c_void,
    data_ptr_ptr: *mut *const u8,
    data_len_ptr: *mut usize,
) {
    let object = unsafe { &*(object_ptr as *const BurkazObject) };
    let Some(object_bytes) = object.to_bytes() else {
        return;
    };

    unsafe {
        *data_ptr_ptr = object_bytes.as_ptr();
        *data_len_ptr = object_bytes.len();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_object(object_ptr: *const c_void) {
    if !object_ptr.is_null() {
        drop(unsafe { Box::from_raw(object_ptr.cast_mut()) });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_create() -> *const c_void {
    BurkazObject::default().into_raw().cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_int(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut i64,
) -> bool {
    let object = unsafe { &*(object_ptr as *const BurkazObject) };
    let mut values = object.field_values(field_id);
    let value_opt = values.next().and_then(|value| value.as_int());
    if let Some(value) = value_opt {
        unsafe { *value_ptr = value };
        true
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_text(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut *const c_char,
) -> bool {
    let object = unsafe { &*(object_ptr as *const BurkazObject) };
    let mut values = object.field_values(field_id);
    let value_opt = values.next().and_then(|value| value.as_text());
    if let Some(value) = value_opt {
        // Allocate a C string so the caller receives a valid null-terminated string.
        // Caller is responsible for freeing via `burkaz_free_string`.
        if let Ok(c_string) = CString::new(value) {
            let ptr = c_string.into_raw();
            unsafe { *value_ptr = ptr };
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_int(object_ptr: *const c_void, field_id: u32, value: i64) {
    let object = unsafe { &mut *(object_ptr as *mut BurkazObject) };
    object.write_value(field_id, &tantivy::schema::OwnedValue::I64(value));
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_text(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *const c_char,
) {
    let object = unsafe { &mut *(object_ptr as *mut BurkazObject) };
    let value = unsafe { CStr::from_ptr(value_ptr).to_str().unwrap().to_string() };

    object.write_value(field_id, &tantivy::schema::OwnedValue::Str(value));
}
