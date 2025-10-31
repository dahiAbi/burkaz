use std::ffi::{CStr, c_char, c_void};

use crate::{query::BurkazQuery, schema::BurkazIndexingStrategy, term_from_ptr};

#[macro_export]
macro_rules! query_from_ptr {
    ($ptr:expr) => {
        unsafe { crate::query::BurkazQuery::from_raw($ptr as *mut _) }
    };
}

macro_rules! query_into_raw {
    ($query:expr) => {
        Box::new($query).into_raw().cast()
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_empty_query() -> *const c_void {
    let query = BurkazQuery::Empty;
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_all_query() -> *const c_void {
    let query = BurkazQuery::All;
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_and_query(
    query_arr_ptr: *const *const c_void,
    query_arr_len: usize,
) -> *const c_void {
    let queries = unsafe { std::slice::from_raw_parts(query_arr_ptr, query_arr_len) };
    let queries = queries
        .iter()
        .map(|query| query_from_ptr!(*query))
        .collect();
    let query = BurkazQuery::And(queries);
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_or_query(
    query_arr_ptr: *const *const c_void,
    query_arr_len: usize,
) -> *const c_void {
    let queries = unsafe { std::slice::from_raw_parts(query_arr_ptr, query_arr_len) };
    let queries = queries
        .iter()
        .map(|query| query_from_ptr!(*query))
        .collect();
    let query = BurkazQuery::Or(queries);
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_not_query(query_ptr: *const c_void) -> *const c_void {
    let query = unsafe { Box::from_raw(query_ptr as *mut BurkazQuery) };
    let query = BurkazQuery::Not(query);
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_boost_query(query_ptr: *const c_void, boost: f32) -> *const c_void {
    let query = unsafe { Box::from_raw(query_ptr as *mut BurkazQuery) };
    let query = BurkazQuery::Boost { query, boost };
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_term_query(
    term_ptr: *const c_void,
    indexing_strategy: u8,
) -> *const c_void {
    if term_ptr.is_null() {
        return burkaz_empty_query();
    }
    let term = term_from_ptr!(term_ptr);

    let query = BurkazQuery::Term {
        term,
        indexing_strategy: BurkazIndexingStrategy::from_code(indexing_strategy).unwrap_or_default(),
    };

    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_fuzzy_term_query(
    term_ptr: *const c_void,
    distance: u8,
    transposition_cost_one: bool,
    prefix: bool,
) -> *const c_void {
    if term_ptr.is_null() {
        return burkaz_empty_query();
    }
    let term = term_from_ptr!(term_ptr);

    let query = BurkazQuery::FuzzyTerm {
        term,
        distance,
        transposition_cost_one,
        prefix,
    };

    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_term_set_query(
    term_arr_ptr: *const *const c_void,
    term_arr_len: usize,
) -> *const c_void {
    let terms = unsafe { &*(std::slice::from_raw_parts(term_arr_ptr, term_arr_len)) };
    let terms = terms.iter().map(|term| term_from_ptr!(*term)).collect();
    let query = BurkazQuery::TermSet { terms };
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_phrase_query(
    term_arr_ptr: *const *const c_void,
    term_arr_len: usize,
    slop: u32,
) -> *const c_void {
    let terms = unsafe { &*(std::slice::from_raw_parts(term_arr_ptr, term_arr_len)) };
    let terms = terms.iter().map(|term| term_from_ptr!(*term)).collect();
    let query = BurkazQuery::Phase { terms, slop };
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_regex_phrase_query(
    field_id: u32,
    term_arr_ptr: *const *const c_char,
    term_arr_len: usize,
    slop: u32,
    max_expansions: u32,
) -> *const c_void {
    let terms = unsafe { &*(std::slice::from_raw_parts(term_arr_ptr, term_arr_len)) };
    let terms = terms
        .iter()
        .map(|ptr| unsafe { CStr::from_ptr(*ptr).to_str().unwrap().to_owned() })
        .collect();
    let query = BurkazQuery::RegexPhase {
        field_id,
        terms,
        slop,
        max_expansions,
    };
    query_into_raw!(query)
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_parse_query(
    query_text_ptr: *const c_char,
    query_text_len: usize,
) -> *const c_void {
    let query_text = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(query_text_ptr.cast(), query_text_len))
    };
    let query = BurkazQuery::Parse { query_text: query_text.to_owned() };
    query_into_raw!(query)
}
