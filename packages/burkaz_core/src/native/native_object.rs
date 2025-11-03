use std::ffi::{CStr, CString, c_char, c_void};

use tantivy::{TantivyDocument, schema::Value};

macro_rules! tantivy_doc_from_ptr {
    ($ptr:expr) => {
        unsafe { &*($ptr as *const tantivy::TantivyDocument) }
    };
}

macro_rules! tantivy_doc_from_ptr_mut {
    ($ptr:expr) => {
        unsafe { &mut *($ptr as *mut tantivy::TantivyDocument) }
    };
}

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
    let object = tantivy_doc_from_ptr!(object_ptr);
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
pub extern "C" fn burkaz_object_read_int_list(
    object_ptr: *const c_void,
    field_id: u32,
    result_arr_ptr: *mut *const i64,
    result_arr_len_ptr: *mut usize,
) -> bool {
    let object = tantivy_doc_from_ptr!(object_ptr);
    let values = object
        .get_all(tantivy::schema::Field::from_field_id(field_id))
        .filter_map(|value| value.as_i64())
        .collect::<Vec<_>>();
    let len = values.len();
    unsafe {
        let values_boxed_slice = values.into_boxed_slice();
        *result_arr_ptr = Box::into_raw(values_boxed_slice).cast();
        *result_arr_len_ptr = len;
    }
    len != 0
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_free_result_array(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_boolean(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut bool,
) -> bool {
    let object = tantivy_doc_from_ptr!(object_ptr);
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
pub extern "C" fn burkaz_object_read_boolean_list(
    object_ptr: *const c_void,
    field_id: u32,
    result_arr_ptr: *mut *const bool,
    result_arr_len_ptr: *mut usize,
) -> bool {
    let object = tantivy_doc_from_ptr!(object_ptr);
    let values = object
        .get_all(tantivy::schema::Field::from_field_id(field_id))
        .filter_map(|value| value.as_bool())
        .collect::<Vec<_>>();
    let len = values.len();
    unsafe {
        let values_boxed_slice = values.into_boxed_slice();
        *result_arr_ptr = Box::into_raw(values_boxed_slice).cast();
        *result_arr_len_ptr = len;
    }
    len != 0
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_read_text(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *mut *const c_char,
) -> bool {
    let object = tantivy_doc_from_ptr!(object_ptr);
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
pub extern "C" fn burkaz_object_read_text_list(
    object_ptr: *const c_void,
    field_id: u32,
    result_arr_ptr: *mut *const c_char,
    result_arr_len_ptr: *mut usize,
) -> bool {
    let object = tantivy_doc_from_ptr!(object_ptr);
    let values = object
        .get_all(tantivy::schema::Field::from_field_id(field_id))
        .filter_map(move |value| {
            value.as_str().and_then(move |value| {
                CString::new(value)
                    .ok()
                    .map(move |c_value| c_value.into_raw())
            })
        })
        .collect::<Vec<_>>();
    let len = values.len();
    unsafe {
        let values_boxed_slice = values.into_boxed_slice();
        *result_arr_ptr = Box::into_raw(values_boxed_slice).cast();
        *result_arr_len_ptr = len;
    }
    len != 0
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_int(object_ptr: *const c_void, field_id: u32, value: i64) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    object.add_i64(tantivy::schema::Field::from_field_id(field_id), value);
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_int_list(
    object_ptr: *const c_void,
    field_id: u32,
    value_arr_ptr: *const i64,
    value_arr_len: usize,
) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    let field = tantivy::schema::Field::from_field_id(field_id);
    let values = unsafe { std::slice::from_raw_parts(value_arr_ptr, value_arr_len) };
    for value in values {
        object.add_i64(field, *value);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_boolean(
    object_ptr: *const c_void,
    field_id: u32,
    value: bool,
) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    object.add_bool(tantivy::schema::Field::from_field_id(field_id), value);
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_boolean_list(
    object_ptr: *const c_void,
    field_id: u32,
    value_arr_ptr: *const bool,
    value_arr_len: usize,
) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    let field = tantivy::schema::Field::from_field_id(field_id);
    let values = unsafe { std::slice::from_raw_parts(value_arr_ptr, value_arr_len) };
    for value in values {
        object.add_bool(field, *value);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_text(
    object_ptr: *const c_void,
    field_id: u32,
    value_ptr: *const c_char,
) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    let value = unsafe { CStr::from_ptr(value_ptr).to_str().unwrap().to_string() };

    object.add_text(tantivy::schema::Field::from_field_id(field_id), value);
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_object_write_text_list(
    object_ptr: *const c_void,
    field_id: u32,
    value_arr_ptr: *const *const c_char,
    value_arr_len: usize,
) {
    let object = tantivy_doc_from_ptr_mut!(object_ptr);
    let field = tantivy::schema::Field::from_field_id(field_id);
    let values = unsafe { std::slice::from_raw_parts(value_arr_ptr, value_arr_len) };
    for value in values {
        let value = unsafe { CStr::from_ptr(*value).to_str().unwrap().to_string() };
        object.add_text(field, value);
    }
}
