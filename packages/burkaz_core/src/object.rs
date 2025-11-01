use std::{
    borrow::Cow,
    io::{Read, Write},
    ops::Range,
};

use byteorder::{ReadBytesExt, WriteBytesExt};
use tantivy::{
    Document,
    schema::{
        Field, OwnedValue, Value,
        document::{DocumentDeserialize, ReferenceValue, ReferenceValueLeaf},
    },
};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueType {
    #[default]
    Null = 0,
    Int64 = 1,
    Text = 2,
    Boolean = 3,
}

type Addr = u32;

#[derive(Debug, Clone, Default)]
pub struct BurkazObject {
    header: Vec<(u32, Range<Addr>)>,
    bytes: Vec<u8>,
}

impl BurkazObject {
    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        unsafe { *Box::from_raw(ptr) }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut object = BurkazObject::default();

        let mut reader = std::io::Cursor::new(bytes);

        let header_len = u64::deserialize(&mut reader).ok()?;
        for _ in 0..header_len {
            let field_id = u32::deserialize(&mut reader).ok()?;
            let range_start = u32::deserialize(&mut reader).ok()?;
            let range_end = u32::deserialize(&mut reader).ok()?;
            object.header.push((field_id, range_start..range_end));
        }

        object
            .bytes
            .extend_from_slice(&reader.get_ref().get(reader.position() as usize..)?);

        Some(object)
    }

    pub fn to_bytes(&self) -> Option<Vec<u8>> {
        let mut writer = Vec::new();

        u64::serialize(&(self.header.len() as u64), &mut writer).ok()?;
        for (field_id, addr_range) in &self.header {
            u32::serialize(field_id, &mut writer).ok()?;
            u32::serialize(&(addr_range.start as u32), &mut writer).ok()?;
            u32::serialize(&(addr_range.end as u32), &mut writer).ok()?;
        }

        writer.extend_from_slice(&self.bytes);

        Some(writer)
    }

    pub fn field_values(&self, field_id: u32) -> impl Iterator<Item = BurkazValueRef<'_>> {
        self.header.iter().filter_map(move |(id, addr)| {
            if id == &field_id {
                let value_bytes = self.bytes.get(addr.start as usize..addr.end as usize)?;
                Some(BurkazValueRef(value_bytes))
            } else {
                None
            }
        })
    }

    pub fn write_value(&mut self, field_id: u32, value: &OwnedValue) {
        let mut value_bytes = vec![];

        macro_rules! write_type {
            ($type:ident) => {{
                value_bytes.push(ValueType::$type as u8);
            }};
        }

        macro_rules! write_value {
            ($value:expr) => {{
                if let Err(_) = BinarySerializable::serialize($value, &mut value_bytes) {
                    return;
                }
            }};
        }

        match value {
            OwnedValue::Null => write_type!(Null),
            OwnedValue::I64(value) => {
                write_type!(Int64);
                write_value!(value);
            }
            OwnedValue::Str(value) => {
                write_type!(Text);
                write_value!(value);
            }
            OwnedValue::Bool(value) => {
                write_type!(Boolean);
                write_value!(value);
            }
            _ => return,
        }

        let start = self.bytes.len();
        let end = start + value_bytes.len();
        self.header.push((field_id, start as Addr..end as Addr));
        self.bytes.extend_from_slice(&value_bytes);
    }
}

impl Document for BurkazObject {
    type Value<'a> = BurkazValueRef<'a>;
    type FieldsValuesIter<'a> = BurkazFieldValuesIter<'a>;

    fn iter_fields_and_values(&self) -> Self::FieldsValuesIter<'_> {
        BurkazFieldValuesIter::new(self)
    }
}

impl DocumentDeserialize for BurkazObject {
    fn deserialize<'de, D>(
        mut deserializer: D,
    ) -> Result<Self, tantivy::schema::document::DeserializeError>
    where
        D: tantivy::schema::document::DocumentDeserializer<'de>,
    {
        let mut object = BurkazObject::default();

        while let Ok(Some((field, value))) = deserializer.next_field::<OwnedValue>() {
            object.write_value(field.field_id(), &value);
        }

        Ok(object)
    }
}

pub struct BurkazFieldValuesIter<'a> {
    object: &'a BurkazObject,
    header_iter: std::slice::Iter<'a, (u32, Range<Addr>)>,
}

impl<'a> BurkazFieldValuesIter<'a> {
    pub fn new(object: &'a BurkazObject) -> Self {
        Self {
            object,
            header_iter: object.header.iter(),
        }
    }
}

impl<'a> Iterator for BurkazFieldValuesIter<'a> {
    type Item = (Field, BurkazValueRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.header_iter.next() {
            Some((id, addr_range)) => {
                let value_bytes = self
                    .object
                    .bytes
                    .get(addr_range.start as usize..addr_range.end as usize)?;
                Some((Field::from_field_id(*id), BurkazValueRef(value_bytes)))
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BurkazValueRef<'a>(&'a [u8]);

impl<'a> BurkazValueRef<'a> {
    pub const fn wrap(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    pub fn typ(&self) -> ValueType {
        let type_code = self.0.as_ref()[0];
        match type_code {
            0 => ValueType::Null,
            1 => ValueType::Int64,
            2 => ValueType::Text,
            3 => ValueType::Boolean,
            _ => ValueType::Null,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.typ() != ValueType::Int64 {
            return None;
        }
        Some(read_i64_le_from_bytes(self.0.as_ref(), 1)?)
    }

    pub fn as_text(&self) -> Option<&'a str> {
        if self.typ() != ValueType::Text {
            return None;
        }
        let len = read_u64_le_from_bytes(self.0.as_ref(), 1)? as usize;
        let str_bytes = self.0.as_ref().get(9..9 + len)?;
        Some(unsafe { std::str::from_utf8_unchecked(str_bytes) })
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.typ() != ValueType::Boolean {
            return None;
        }
        Some(self.0.as_ref().get(1)? == &1u8)
    }
}

impl<'a> Value<'a> for BurkazValueRef<'a> {
    type ArrayIter = EmptyArrayIter<'a>;
    type ObjectIter = EmptyObjectIter<'a>;

    fn as_value(&self) -> ReferenceValue<'a, Self> {
        match self.typ() {
            ValueType::Null => ReferenceValueLeaf::Null.into(),
            ValueType::Int64 => ReferenceValueLeaf::I64(self.as_int().unwrap()).into(),
            ValueType::Text => ReferenceValueLeaf::Str(self.as_text().unwrap()).into(),
            ValueType::Boolean => ReferenceValueLeaf::Bool(self.as_bool().unwrap()).into(),
        }
    }
}

pub struct EmptyArrayIter<'a> {
    _p: std::marker::PhantomData<&'a ()>,
}
impl<'a> Iterator for EmptyArrayIter<'a> {
    type Item = BurkazValueRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct EmptyObjectIter<'a> {
    _p: std::marker::PhantomData<&'a ()>,
}
impl<'a> Iterator for EmptyObjectIter<'a> {
    type Item = (&'a str, BurkazValueRef<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn read_u64_le_from_bytes(bytes: &[u8], offset: usize) -> Option<u64> {
    let end = offset.checked_add(8)?;
    let value_bytes = bytes.get(offset..end)?;
    Some(u64::from_le_bytes(unsafe { *value_bytes.as_ptr().cast() }))
}

fn read_i64_le_from_bytes(bytes: &[u8], offset: usize) -> Option<i64> {
    let end = offset.checked_add(8)?;
    let value_bytes = bytes.get(offset..end)?;
    Some(i64::from_le_bytes(unsafe { *value_bytes.as_ptr().cast() }))
}

pub trait BinarySerializable: std::fmt::Debug + Sized {
    fn serialize<W: Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()>;
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self>;
}

impl BinarySerializable for () {
    fn serialize<W: Write + ?Sized>(&self, _: &mut W) -> std::io::Result<()> {
        Ok(())
    }
    fn deserialize<R: Read>(_: &mut R) -> std::io::Result<Self> {
        Ok(())
    }
}

impl BinarySerializable for ValueType {
    fn serialize<W: Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        (*self as u8).serialize(writer)
    }

    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        const VALUE_TYPE_COUNT: u8 = 3;
        let num = u8::deserialize(reader)?;
        let type_code = if (0..=VALUE_TYPE_COUNT).contains(&num) {
            unsafe { std::mem::transmute::<u8, ValueType>(num) }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid value type id: {num}"),
            ));
        };
        Ok(type_code)
    }
}

impl<T: BinarySerializable> BinarySerializable for Vec<T> {
    fn serialize<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        (self.len() as u64).serialize(writer)?;
        for it in self {
            it.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let num_items = u64::deserialize(reader)?;
        let mut items = Vec::<T>::with_capacity(num_items as usize);
        for _ in 0..num_items {
            let item = T::deserialize(reader)?;
            items.push(item);
        }
        Ok(items)
    }
}

impl BinarySerializable for u8 {
    #[inline]
    fn serialize<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(*self)
    }

    #[inline]
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u8()
    }
}

impl BinarySerializable for bool {
    #[inline]
    fn serialize<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(*self as u8)
    }

    #[inline]
    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let value = u8::deserialize(reader)?;
        Ok(value == 1u8)
    }
}

macro_rules! impl_numeric_binary_serializable {
    ($type:ty, $size:expr, $write_method:ident, $read_method:ident) => {
        impl BinarySerializable for $type {
            #[inline]
            fn serialize<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
                byteorder::WriteBytesExt::$write_method::<byteorder::LittleEndian>(writer, *self)
            }

            #[inline]
            fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                byteorder::ReadBytesExt::$read_method::<byteorder::LittleEndian>(reader)
            }
        }
    };
}

impl_numeric_binary_serializable!(u16, 2, write_u16, read_u16);
impl_numeric_binary_serializable!(u32, 4, write_u32, read_u32);
impl_numeric_binary_serializable!(u64, 8, write_u64, read_u64);
impl_numeric_binary_serializable!(i64, 8, write_i64, read_i64);
impl_numeric_binary_serializable!(f64, 8, write_f64, read_f64);

impl BinarySerializable for String {
    fn serialize<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        let data = self.as_bytes();
        (data.len() as u64).serialize(writer)?;
        writer.write_all(data)
    }

    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let len = u64::deserialize(reader)?;
        let mut result = String::with_capacity(len as usize);
        reader.take(len).read_to_string(&mut result)?;
        Ok(result)
    }
}

impl<'a> BinarySerializable for Cow<'a, str> {
    fn serialize<W: Write + ?Sized>(&self, writer: &mut W) -> std::io::Result<()> {
        let data = self.as_bytes();
        (data.len() as u64).serialize(writer)?;
        writer.write_all(data)
    }

    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let len = u64::deserialize(reader)?;
        let mut result = String::with_capacity(len as usize);
        reader.take(len).read_to_string(&mut result)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burkaz_value_ref_typ() {
        let value = BurkazValueRef(&[0]);
        assert_eq!(value.typ(), ValueType::Null);
        let value = BurkazValueRef(&[1]);
        assert_eq!(value.typ(), ValueType::Int64);
        let value = BurkazValueRef(&[2]);
        assert_eq!(value.typ(), ValueType::Text);
        let value = BurkazValueRef(&[3]);
        assert_eq!(value.typ(), ValueType::Boolean);
        let value = BurkazValueRef(&[4]);
        assert_eq!(value.typ(), ValueType::Null); // invalid type code
    }

    #[test]
    fn burkaz_value_ref_as_int() {
        let bytes = &[1, 0, 0, 0, 0, 0, 0, 0, 0];
        let value = BurkazValueRef(bytes);
        assert_eq!(value.typ(), ValueType::Int64);
        assert_eq!(value.as_int(), Some(0));
    }

    #[test]
    fn burkaz_value_ref_as_text() {
        let bytes = &[2, 0, 0, 0, 0, 0, 0, 0, 0];
        let value = BurkazValueRef(bytes);
        assert_eq!(value.typ(), ValueType::Text);
        assert_eq!(value.as_text(), Some(""));

        let bytes_2 = &[2, 1, 0, 0, 0, 0, 0, 0, 0, 95];
        let value_2 = BurkazValueRef(bytes_2);
        assert_eq!(value_2.typ(), ValueType::Text);
        assert_eq!(
            value_2.as_text(),
            Some(unsafe { str::from_utf8_unchecked(&[95]) })
        );
    }
}
