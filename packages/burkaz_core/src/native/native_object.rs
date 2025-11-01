use std::ffi::{CStr, CString, c_char, c_void};

use tantivy::{TantivyDocument, schema::Value};

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_object(object_ptr: *const c_void) {
    if !object_ptr.is_null() {
        drop(unsafe { Box::from_raw(object_ptr.cast_mut()) });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_create() -> *const c_void {
    Box::into_raw(Box::new(TantivyDocument::new())).cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_int(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut i64,
) -> bool {
    let object = unsafe { &*(object_ptr as *const TantivyDocument) };
    let mut values = object.get_all(tantivy::schema::Field::from_field_id(field_id));
    let value_opt = values.next().and_then(|value| value.as_i64());
    if let Some(value) = value_opt {
        unsafe { *value_ptr = value };
        true
    } else {
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_boolean(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut bool,
) -> bool {
    let object = unsafe { &*(object_ptr as *const TantivyDocument) };
    let mut values = object.get_all(tantivy::schema::Field::from_field_id(field_id));
    let value_opt = values.next().and_then(|value| value.as_bool());
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
    let object = unsafe { &*(object_ptr as *const TantivyDocument) };
    let mut values = object.get_all(tantivy::schema::Field::from_field_id(field_id));
    let value_opt = values.next().and_then(|value| value.as_str());
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
    let object = unsafe { &mut *(object_ptr as *mut TantivyDocument) };
    object.add_i64(tantivy::schema::Field::from_field_id(field_id), value);
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_boolean(object_ptr: *const c_void, field_id: u32, value: bool) {
    let object = unsafe { &mut *(object_ptr as *mut TantivyDocument) };
    object.add_bool(tantivy::schema::Field::from_field_id(field_id), value);
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_text(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *const c_char,
) {
    let object = unsafe { &mut *(object_ptr as *mut TantivyDocument) };
    let value = unsafe { CStr::from_ptr(value_ptr).to_str().unwrap().to_string() };

    object.add_text(tantivy::schema::Field::from_field_id(field_id), value);
}
