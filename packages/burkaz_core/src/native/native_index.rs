use std::{
    ffi::{CString, c_char, c_void},
    path::Path,
};

use tantivy::TantivyDocument;

use crate::{
    error::BurkazError,
    index::{BurkazDirectory, BurkazIndex},
    schema::BurkazSchema,
    str_from_ptr,
};

#[macro_export]
macro_rules! index_from_ptr {
    ($index_ptr:expr) => {
        unsafe { (&*($index_ptr as *mut crate::index::BurkazIndex)) }
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_open(
    name_ptr: *const c_char,
    name_len: usize,
    directory_path_ptr: *const c_char,
    directory_path_len: usize,
    schema_ptr: *const c_void,
    index_ptr_ptr: *mut *const c_void,
) -> u8 {
    catch_error!({
        let name = if !name_ptr.is_null() {
            str_from_ptr!(name_ptr, name_len)
        } else {
            return Err(BurkazError::NullPointer("name pointer is null"));
        };

        let schema = if !schema_ptr.is_null() {
            unsafe { BurkazSchema::from_raw(schema_ptr as *mut _) }
        } else {
            return Err(BurkazError::NullPointer("schema pointer is null"));
        };

        let directory_path = if !directory_path_ptr.is_null() {
            Some(Path::new(str_from_ptr!(
                directory_path_ptr,
                directory_path_len
            )))
        } else {
            None
        };

        let directory = match directory_path {
            Some(path) => BurkazDirectory::OnDisk(&path.join(&name)),
            None => BurkazDirectory::InMemory,
        };

        directory.create_if_not_exists()?;

        let index = BurkazIndex::new(name.to_owned(), schema, directory)?;

        unsafe {
            *index_ptr_ptr = index.into_raw().cast();
        }

        ok!()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_close(index_ptr: *const c_void) {
    if !index_ptr.is_null() {
        let index = unsafe { BurkazIndex::from_raw(index_ptr as *mut _) };

        // close the index

        drop(index);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_name(index_ptr: *const c_void) -> *const c_char {
    let index = index_from_ptr!(index_ptr);

    if let Ok(name) = CString::new(index.name()) {
        name.into_raw().cast()
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_string(string_ptr: *const c_char) {
    if !string_ptr.is_null() {
        drop(unsafe { CString::from_raw(string_ptr.cast_mut()) });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_add(index_ptr: *const c_void, object_ptr: *const c_void) -> u8 {
    let object = unsafe { *Box::<TantivyDocument>::from_raw(object_ptr as *mut _) };

    catch_error!({
        let index = index_from_ptr!(index_ptr);
        let result = index.add(object);
        result
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_add_all(
    index_ptr: *const c_void,
    object_array_ptr: *const *const c_void,
    object_array_len: usize,
) -> u8 {
    catch_error!({
        let objects = unsafe {
            if object_array_ptr.is_null() {
                Vec::new()
            } else {
                std::slice::from_raw_parts(object_array_ptr, object_array_len)
                    .iter()
                    .map(|object_ptr| *Box::<TantivyDocument>::from_raw(*object_ptr as *mut _))
                    .collect()
            }
        };

        let index = index_from_ptr!(index_ptr);
        index.add_all(objects)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_clear(index_ptr: *const c_void) -> u8 {
    catch_error!({
        let index = index_from_ptr!(index_ptr);
        index.clear()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_index_get(
    index_ptr: *const c_void,
    addr: u64,
    object_ptr: *mut *const c_void,
) -> u8 {
    catch_error!({
        let index = index_from_ptr!(index_ptr);
        let result = index.get(addr.into());

        let object = result?;

        unsafe {
            *object_ptr = Box::into_raw(Box::new(object)).cast();
        }

        ok!()
    })
}
