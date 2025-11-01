use std::ffi::{c_char, c_void};

use crate::{
    schema::{
        BurkazIndexingStrategy, BurkazSchema, BurkazSchemaField, BurkazSchemaFieldOptions,
        BurkazSchemaFieldType,
    },
    str_from_ptr,
};

#[repr(C)]
pub struct CBurkazSchemaFieldOptions {
    pub typ: u8,
    pub stored: u8,
    pub coerce: u8,
    pub indexed: u8,
    pub fieldnorms: u8,
    pub fast: u8,
    pub indexing_strategy: u8,
    pub fast_tokenizer_ptr: *const c_char,
    pub fast_tokenizer_len: usize,
    pub indexing_tokenizer_ptr: *const c_char,
    pub indexing_tokenizer_len: usize,
}

#[repr(C)]
pub struct CBurkazSchemaField {
    pub name_ptr: *const c_char,
    pub name_len: usize,
    pub options_ptr: *const CBurkazSchemaFieldOptions,
}

#[unsafe(no_mangle)]
pub extern "C" fn burkaz_schema_new(
    field_array_ptr: *const *const CBurkazSchemaField,
    field_array_len: usize,
) -> *const c_void {
    let mut schema = BurkazSchema::default();

    if !field_array_ptr.is_null() {
        for i in 0..field_array_len {
            let c_field = unsafe { &*(*field_array_ptr.wrapping_add(i)) };
            let name = str_from_ptr!(c_field.name_ptr, c_field.name_len).to_owned();

            let c_options = unsafe { &*c_field.options_ptr };

            let Some(options) = options_from_native(c_options) else {
                continue;
            };

            let field = BurkazSchemaField::new(name, options);

            schema.add_field(field);
        }
    }

    schema.into_raw().cast()
}

fn options_from_native(c_options: &CBurkazSchemaFieldOptions) -> Option<BurkazSchemaFieldOptions> {
    let typ = match c_options.typ {
        1 => BurkazSchemaFieldType::Int64,
        2 => BurkazSchemaFieldType::Text,
        3 => BurkazSchemaFieldType::Boolean,
        _ => return None,
    };

    let indexing_strategy = BurkazIndexingStrategy::from_code(c_options.indexing_strategy);

    macro_rules! as_bool {
        ($value:expr) => {
            match $value as u8 {
                1 => true,
                0 => false,
                _ => return None,
            }
        };
    }

    macro_rules! as_optional_string {
        ($ptr:expr, $len:expr) => {
            if $ptr.is_null() {
                None
            } else {
                Some(str_from_ptr!($ptr, $len).to_owned())
            }
        };
    }

    Some(BurkazSchemaFieldOptions {
        typ: typ,
        fast: as_bool!(c_options.fast),
        stored: as_bool!(c_options.stored),
        coerce: as_bool!(c_options.coerce),
        indexed: as_bool!(c_options.indexed),
        fieldnorms: as_bool!(c_options.fieldnorms),
        indexing_strategy: indexing_strategy,
        fast_tokenizer: as_optional_string!(
            c_options.fast_tokenizer_ptr,
            c_options.fast_tokenizer_len
        ),
        indexing_tokenizer: as_optional_string!(
            c_options.indexing_tokenizer_ptr,
            c_options.indexing_tokenizer_len
        ),
    })
}
