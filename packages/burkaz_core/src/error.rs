use std::ffi::{CString, c_char};
use std::sync::{Mutex, OnceLock};

use crate::address::BurkazObjectAddr;

/// Thread-local error storage using Mutex for safe concurrent access
static LAST_ERROR: OnceLock<Mutex<Option<String>>> = OnceLock::new();

/// Gets the error storage mutex, initializing it if needed
#[inline]
fn get_error_storage() -> &'static Mutex<Option<String>> {
    LAST_ERROR.get_or_init(|| Mutex::new(None))
}

/// Stores an error message in the global error storage
pub fn store_error<E: std::fmt::Display>(error: E) {
    if let Ok(mut error_storage) = get_error_storage().lock() {
        *error_storage = Some(error.to_string());
    }
}

/// Retrieves the last stored error message
pub fn get_last_error() -> Option<String> {
    if let Ok(mut error_storage) = get_error_storage().lock() {
        error_storage.take()
    } else {
        None
    }
}

/// Macro to catch Result errors and return boolean (uint8) indicating success/failure
#[macro_export]
macro_rules! catch_error {
    ($block:block) => {{
        let result = || $block;

        match result() as Result<(), _> {
            Ok(_) => 0u8,
            Err(error) => {
                $crate::error::store_error(error);
                1u8
            }
        }
    }};
}

#[macro_export]
macro_rules! ok {
    () => {
        Result::<(), $crate::error::BurkazError>::Ok(())
    };
}

/// Native function to get the last error as a C string
/// Returns a pointer to a null-terminated string, or null if no error
/// The caller is responsible for freeing the returned string
#[unsafe(no_mangle)]
pub extern "C" fn burkaz_get_last_error() -> *mut c_char {
    match get_last_error() {
        Some(error) => match CString::new(error) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        None => std::ptr::null_mut(),
    }
}

/// Native function to free a C string returned by get_last_error_c
#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_error_string(s: *mut c_char) {
    if !s.is_null() {
        drop(unsafe { CString::from_raw(s) });
    }
}

pub enum BurkazError {
    TantivyError(tantivy::TantivyError),
    UnknownError(String),
    NullPointer(&'static str),
    ObjectNotFound(BurkazObjectAddr),
}

impl From<BurkazObjectAddr> for BurkazError {
    fn from(addr: BurkazObjectAddr) -> Self {
        BurkazError::ObjectNotFound(addr)
    }
}

impl From<tantivy::TantivyError> for BurkazError {
    fn from(error: tantivy::TantivyError) -> Self {
        BurkazError::TantivyError(error)
    }
}

impl std::fmt::Display for BurkazError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BurkazError::TantivyError(error) => write!(f, "Tantivy error: {}", error),
            BurkazError::UnknownError(error) => write!(f, "Unknown error: {}", error),
            BurkazError::NullPointer(error) => write!(f, "Null pointer error: {}", error),
            BurkazError::ObjectNotFound(addr) => write!(f, "Object not found: {}", addr),
        }
    }
}

impl std::fmt::Debug for BurkazError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BurkazError::TantivyError(error) => write!(f, "BurkazError::TantivyError({:?})", error),
            BurkazError::UnknownError(error) => write!(f, "BurkazError::UnknownError({:?})", error),
            BurkazError::NullPointer(error) => write!(f, "BurkazError::NullPointer({:?})", error),
            BurkazError::ObjectNotFound(addr) => write!(f, "BurkazError::ObjectNotFound({:?})", addr),
        }
    }
}
