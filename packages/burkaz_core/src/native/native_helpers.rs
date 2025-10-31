
#[macro_export]
macro_rules! str_from_ptr {
    ($ptr:expr, $len:expr) => {
        unsafe { str::from_utf8_unchecked(std::slice::from_raw_parts($ptr.cast(), $len)) }
    };
}
