#![allow(unused)]

use std::borrow::Cow;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use tantivy::{columnar::MonotonicallyMappableToU64, schema::document::ReferenceValueLeaf};

use crate::object::{BinarySerializable, BurkazValueRef};

#[derive(Debug, Clone)]
pub struct BurkazTerm<B = Vec<u8>>(B)
where
    B: AsRef<[u8]>;

impl BurkazTerm<Vec<u8>> {
    pub fn new(field_id: u32, value: &ReferenceValueLeaf<'_>) -> Self {
        let mut bytes = Vec::<u8>::new();
        bytes.write_u32::<LittleEndian>(field_id).unwrap();

        macro_rules! write_type {
            ($type:ident) => {{
                let _ = u8::serialize(&(crate::object::ValueType::$type as u8), &mut bytes);
            }};
        }

        macro_rules! write_value {
            ($type:ident, $value_type:ty, $value:expr) => {{
                write_type!($type);
                let _ = <$value_type as crate::object::BinarySerializable>::serialize(
                    $value, &mut bytes,
                );
            }};
        }

        match value {
            ReferenceValueLeaf::I64(value) => write_value!(Int64, i64, value),
            ReferenceValueLeaf::Bool(value) => write_value!(Boolean, bool, value),
            ReferenceValueLeaf::Str(value) => write_value!(Text, Cow<str>, &Cow::Borrowed(value)),
            _ => unimplemented!("Unsupported value: {:?}", value),
        }
        Self(bytes)
    }
}

impl<B> BurkazTerm<B>
where
    B: AsRef<[u8]>,
{
    pub const fn wrap(bytes: B) -> Self {
        Self(bytes)
    }

    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn field_id(&self) -> Option<u32> {
        self.0.as_ref().read_u32::<LittleEndian>().ok()
    }

    pub fn val(&self) -> BurkazValueRef<'_> {
        BurkazValueRef::wrap(&self.0.as_ref()[4..])
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_ref()
    }
}

impl<B> BurkazTerm<B>
where
    B: AsRef<[u8]>,
{
    pub fn to_tantivy_term(&self) -> tantivy::schema::Term {
        let mut bytes = vec![];

        bytes
            .write_u32::<BigEndian>(self.field_id().unwrap())
            .unwrap();

        let val = self.val();

        macro_rules! write_type {
            ($type:ident) => {{
                let _ = u8::serialize(&(tantivy::schema::Type::$type as u8), &mut bytes);
            }};
        }

        macro_rules! write_fast_value {
            ($type:ident, $value:expr) => {{
                write_type!($type);
                bytes.write_u64::<BigEndian>($value.to_u64()).unwrap();
            }};
        }

        let val_type = val.typ();

        match val_type {
            crate::object::ValueType::Int64 => write_fast_value!(I64, val.as_int().unwrap()),
            crate::object::ValueType::Text => {
                write_type!(Str);
                let text_val = val.as_text().unwrap();
                bytes.extend_from_slice(text_val.as_bytes());
            }
            crate::object::ValueType::Boolean => write_fast_value!(Bool, val.as_bool().unwrap()),
            _ => unimplemented!("Unsupported term value type: {:?}", val_type),
        }

        tantivy::schema::Term::wrap(bytes)
    }
}
