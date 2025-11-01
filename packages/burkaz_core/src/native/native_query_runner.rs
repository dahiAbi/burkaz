use std::ffi::c_void;

use crate::{address::BurkazObjectAddr, query::BurkazQuery, query_runner::QueryRunner};

macro_rules! query_runner_from_ptr {
    ($query_runner_ptr:expr) => {
        unsafe { &*($query_runner_ptr as *const crate::query_runner::QueryRunner) }
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_query_runner_new(
    index_ptr: *const c_void,
    query_ptr: *const c_void,
    query_runner_ptr: *mut *const c_void,
) -> u8 {
    catch_error!({
        let index = index_from_ptr!(index_ptr);

        let query = if query_ptr.is_null() {
            BurkazQuery::Empty
        } else {
            query_from_ptr!(query_ptr)
        };

        let query_runner = QueryRunner::new(index.downgrade(), query);

        unsafe {
            *query_runner_ptr = query_runner.into_raw().cast();
        }

        ok!()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_query_runner(query_runner_ptr: *const c_void) {
    if !query_runner_ptr.is_null() {
        drop(unsafe { Box::from_raw(query_runner_ptr.cast_mut()) });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_query_runner_count(
    query_runner_ptr: *const c_void,
    result_ptr: *mut usize,
) -> u8 {
    catch_error!({
        let query_runner = query_runner_from_ptr!(query_runner_ptr);
        let result = query_runner.count()?;
        unsafe {
            *result_ptr = result;
        }
        ok!()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_query_runner_search(
    query_runner_ptr: *const c_void,
    offset: usize,
    limit: usize,
    result_arr_ptr: *mut *const u64,
    result_arr_len_ptr: *mut usize,
) -> u8 {
    catch_error!({
        let query_runner = query_runner_from_ptr!(query_runner_ptr);
        let objects = query_runner.search(offset, limit)?;
        if objects.is_empty() {
            return ok!();
        }
        unsafe {
            let boxed_objects = objects.into_boxed_slice();
            *result_arr_len_ptr = boxed_objects.len();
            *result_arr_ptr = Box::into_raw(boxed_objects).cast();
        }
        ok!()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_free_query_runner_search_result(
    result_arr_ptr: *const u64,
    result_arr_len: usize,
) {
    if !result_arr_ptr.is_null() {
        drop(unsafe {
            Vec::from_raw_parts(
                result_arr_ptr.cast_mut() as *mut BurkazObjectAddr,
                result_arr_len,
                result_arr_len,
            )
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_query_runner_delete_all(query_runner_ptr: *const c_void) -> u8 {
    catch_error!({
        let query_runner = query_runner_from_ptr!(query_runner_ptr);
        query_runner.delete_all()
    })
}
