#![allow(unused)]

use tantivy::schema::{
    FieldEntry, FieldType, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing,
    TextOptions,
};

#[derive(Debug)]
pub struct BurkazSchemaField {
    pub name: String,
    pub options: BurkazSchemaFieldOptions,
}

impl BurkazSchemaField {
    pub const fn new(name: String, options: BurkazSchemaFieldOptions) -> Self {
        Self { name, options }
    }
}

#[derive(Debug)]
pub enum BurkazSchemaFieldType {
    Int64,
    Text,
    Boolean,
}

#[derive(Debug)]
pub struct BurkazSchemaFieldOptions {
    pub typ: BurkazSchemaFieldType,
    pub stored: bool,
    pub coerce: bool,
    pub indexed: bool,
    pub fieldnorms: bool,
    pub fast: bool,
    pub indexing_strategy: Option<BurkazIndexingStrategy>,
    pub fast_tokenizer: Option<String>,
    pub indexing_tokenizer: Option<String>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Default)]
pub enum BurkazIndexingStrategy {
    #[default]
    Basic = 1,
    Frequencies = 2,
    FrequenciesAndPositions = 3,
}

impl BurkazIndexingStrategy {
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(BurkazIndexingStrategy::Basic),
            2 => Some(BurkazIndexingStrategy::Frequencies),
            3 => Some(BurkazIndexingStrategy::FrequenciesAndPositions),
            _ => None,
        }
    }

    pub const fn to_code(self) -> u8 {
        self as u8
    }
}

impl Into<IndexRecordOption> for BurkazIndexingStrategy {
    fn into(self) -> IndexRecordOption {
        match self {
            BurkazIndexingStrategy::Basic => IndexRecordOption::Basic,
            BurkazIndexingStrategy::Frequencies => IndexRecordOption::WithFreqs,
            BurkazIndexingStrategy::FrequenciesAndPositions => {
                IndexRecordOption::WithFreqsAndPositions
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct BurkazSchema {
    fields: Vec<BurkazSchemaField>,
}

impl BurkazSchema {
    pub fn add_field(&mut self, field: BurkazSchemaField) {
        self.fields.push(field);
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }
}

impl Into<Schema> for BurkazSchema {
    fn into(self) -> Schema {
        let mut schema = Schema::builder();
        for field in self.fields {
            schema.add_field(FieldEntry::new(field.name, field.options.into()));
        }
        schema.build()
    }
}

impl Into<FieldType> for BurkazSchemaFieldOptions {
    fn into(self) -> FieldType {
        match self.typ {
            BurkazSchemaFieldType::Int64 => FieldType::I64(self.into()),
            BurkazSchemaFieldType::Text => FieldType::Str(self.into()),
            BurkazSchemaFieldType::Boolean => FieldType::Bool(self.into()),
        }
    }
}

impl Into<NumericOptions> for BurkazSchemaFieldOptions {
    fn into(self) -> NumericOptions {
        let mut options = NumericOptions::default();

        if self.indexed {
            options = options.set_indexed();
        }

        if self.fast {
            options = options.set_fast();
        }

        if self.stored {
            options = options.set_stored();
        }

        if self.fieldnorms {
            options = options.set_fieldnorm();
        }

        if self.coerce {
            options = options.set_coerce();
        }

        options
    }
}

impl Into<TextOptions> for BurkazSchemaFieldOptions {
    fn into(self) -> TextOptions {
        let mut options = TextOptions::default();

        if self.stored {
            options = options.set_stored();
        }

        if self.coerce {
            options = options.set_coerce();
        }

        if self.indexed {
            let mut text_field_indexing = TextFieldIndexing::default();

            if let Some(indexing_strategy) = self.indexing_strategy {
                text_field_indexing =
                    text_field_indexing.set_index_option(indexing_strategy.into());
            }

            if let Some(indexing_tokenizer) = self.indexing_tokenizer {
                text_field_indexing = text_field_indexing.set_tokenizer(&indexing_tokenizer);
            }

            text_field_indexing = text_field_indexing.set_fieldnorms(self.fieldnorms);

            options = options.set_indexing_options(text_field_indexing);
        }

        if self.fast {
            options = options.set_fast(self.fast_tokenizer.as_deref());
        }

        options
    }
}
