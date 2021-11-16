//! TODO
//!
//! Could fix this:
//! https://github.com/serde-rs/json/issues/743

use crate::content::ser::{Content, ContentSerializer};
use crate::{DeserializeAs, SerializeAs};
use serde::de::{DeserializeSeed, EnumAccess, Error, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::ser::{
    Impossible, SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTupleVariant,
};
use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// TODO
#[derive(Debug, Copy, Clone)]
pub struct EnumMap;

impl<T> SerializeAs<Vec<T>> for EnumMap
where
    T: Serialize,
{
    fn serialize_as<S>(source: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(SeqAsMapSerializer(serializer))
    }
}

impl<'de, T> DeserializeAs<'de, Vec<T>> for EnumMap
where
    T: Deserialize<'de>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnumMapVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for EnumMapVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map or enum values")
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Vec::deserialize(SeqDeserializer(map))
            }
        }

        deserializer.deserialize_map(EnumMapVisitor(PhantomData))
    }
}

static END_OF_MAP_IDENTIFIER: &str = "__PRIVATE_END_OF_MAP_MARKER__";

// Serialization code below here

/// Convert a sequence to a map during serialization.
///
/// Only `serialize_seq` is implemented and forwarded to `serialize_map` on the inner `Serializer`.
/// The elements are serialized with [`SerializeSeqElement`].
struct SeqAsMapSerializer<S>(S);

impl<S> Serializer for SeqAsMapSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = SerializeSeqElement<S::SerializeMap>;
    type SerializeTuple = Impossible<S::Ok, S::Error>;
    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;
    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;
    type SerializeMap = Impossible<S::Ok, S::Error>;
    type SerializeStruct = Impossible<S::Ok, S::Error>;
    type SerializeStructVariant = Impossible<S::Ok, S::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let is_human_readable = self.0.is_human_readable();
        self.0
            .serialize_map(len)
            .map(|delegate| SerializeSeqElement {
                delegate,
                is_human_readable,
            })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }
}

/// Serialize a single element but turn the sequence into a map logic.
///
/// It uses [`SerializeEnumAsMapElement`] for the map element serialization.
///
/// The [`Serializer`] implementation handles all the `serialize_*_variant` functions and defers to [`SerializeVariant`] for the more complicated tuple and struct variants.
struct SerializeSeqElement<M> {
    delegate: M,
    is_human_readable: bool,
}

impl<M> SerializeSeq for SerializeSeqElement<M>
where
    M: SerializeMap,
{
    type Ok = M::Ok;
    type Error = M::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(EnumAsMapElementSerializer {
            delegate: &mut self.delegate,
            is_human_readable: self.is_human_readable,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.end()
    }
}

struct EnumAsMapElementSerializer<'a, M> {
    delegate: &'a mut M,
    is_human_readable: bool,
}

impl<'a, M> Serializer for EnumAsMapElementSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = SerializeVariant<'a, M>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = SerializeVariant<'a, M>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.delegate.serialize_entry(variant, &())?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.delegate.serialize_entry(variant, value)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeVariant {
            delegate: self.delegate,
            is_human_readable: self.is_human_readable,
            variant,
            content: Content::TupleStruct(name, Vec::with_capacity(len)),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for EnumMap"))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeVariant {
            delegate: self.delegate,
            is_human_readable: self.is_human_readable,
            variant,
            content: Content::Struct(name, Vec::with_capacity(len)),
        })
    }
}

/// Serialize a struct or tuple variant enum as a map element
///
/// [`SerializeStructVariant`] serializes a struct variant, and [`SerializeTupleVariant`] a tuple variant.
struct SerializeVariant<'a, M> {
    delegate: &'a mut M,
    is_human_readable: bool,
    variant: &'static str,
    content: Content,
}

impl<'a, M> SerializeStructVariant for SerializeVariant<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();

    type Error = M::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // Serialize to a Content type first
        let value: Content = value.serialize(ContentSerializer::new(self.is_human_readable))?;
        if let Content::Struct(_name, fields) = &mut self.content {
            fields.push((key, value));
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.serialize_entry(&self.variant, &self.content)
    }
}

impl<'a, M> SerializeTupleVariant for SerializeVariant<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();

    type Error = M::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // Serialize to a Content type first
        let value: Content = value.serialize(ContentSerializer::new(self.is_human_readable))?;
        if let Content::TupleStruct(_name, fields) = &mut self.content {
            fields.push(value);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.serialize_entry(&self.variant, &self.content)
    }
}

// Below is deserialization code

/// Deserialize the sequence of enum instances.
///
/// The main [`Deserializer`] implementation handles the outer sequence (e.g., `Vec`), while the [`SeqAccess`] implementation is responsible for the inner elements.
struct SeqDeserializer<M>(M);

impl<'de, M> Deserializer<'de> for SeqDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, M> SeqAccess<'de> for SeqDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match seed.deserialize(EnumDeserializer(&mut self.0)) {
            Ok(value) => Ok(Some(value)),
            Err(err) => {
                // Unfortunately we loose the optional aspect of MapAccess, so we need to special case an error value to mark the end of the map.
                if err.to_string().contains(END_OF_MAP_IDENTIFIER) {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

/// Deserialize an enum from a map element
///
/// The [`Deserializer`] implementation is the starting point, which first calls the [`EnumAccess`] methods.
/// The [`EnumAccess`] is used to deserialize the enum variant type of the enum.
/// The [`VariantAccess`] is used to deserialize the value part of the enum.
struct EnumDeserializer<M>(M);

impl<'de, M> Deserializer<'de> for EnumDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_enum("", &[], visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl<'de, M> EnumAccess<'de> for EnumDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;
    type Variant = Self;

    fn variant_seed<T>(mut self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.next_key_seed(seed)? {
            Some(key) => Ok((key, self)),

            // Unfortunately we loose the optional aspect of MapAccess, so we need to special case an error value to mark the end of the map.
            None => Err(Error::custom(END_OF_MAP_IDENTIFIER)),
        }
    }
}

impl<'de, M> VariantAccess<'de> for EnumDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn unit_variant(mut self) -> Result<(), Self::Error> {
        self.0.next_value()
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.0.next_value_seed(seed)
    }

    fn tuple_variant<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.next_value_seed(SeedTupleVariant { len, visitor })
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.next_value_seed(SeedStructVariant { visitor })
    }
}

struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}

impl<'de, V> DeserializeSeed<'de> for SeedTupleVariant<V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(self.len, self.visitor)
    }
}

struct SeedStructVariant<V> {
    visitor: V,
}

impl<'de, V> DeserializeSeed<'de> for SeedStructVariant<V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self.visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Configure;
    use std::net::IpAddr;
    use std::str::FromStr;

    fn bytes_debug_readable(bytes: &[u8]) -> String {
        let mut result = String::with_capacity(bytes.len() * 2);
        for &byte in bytes {
            match byte {
                control if control < 0x20 || control == 0x7f => {
                    result.push_str(&format!("\\x{:02x}", byte));
                }
                non_ascii if non_ascii > 0x7f => {
                    result.push_str(&format!("\\x{:02x}", byte));
                }
                b'\\' => result.push_str("\\\\"),
                _ => {
                    result.push(byte as char);
                }
            }
        }
        result
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    enum EnumValue {
        Int(i32),
        String(String),
        Unit,
        Tuple(i32, String, bool),
        Struct {
            a: i32,
            b: String,
            c: bool,
        },
        Ip(IpAddr, IpAddr),
        #[serde(rename = "$value")]
        Extra(serde_json::Value),
    }

    #[crate::serde_as(crate = "crate")]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct VecEnumValues {
        #[serde_as(as = "EnumMap")]
        vec: Vec<EnumValue>,
    }

    #[test]
    fn json_round_trip() {
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                EnumValue::Int(456),
                EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                EnumValue::Tuple(1, "Middle".to_string(), false),
                EnumValue::Struct {
                    a: 666,
                    b: "BBB".to_string(),
                    c: true,
                },
            ],
        };

        let json = serde_json::to_string_pretty(&values).unwrap();
        expect_test::expect![[r#"
            {
              "vec": {
                "Int": 123,
                "String": "FooBar",
                "Int": 456,
                "String": "XXX",
                "Unit": null,
                "Tuple": [
                  1,
                  "Middle",
                  false
                ],
                "Struct": {
                  "a": 666,
                  "b": "BBB",
                  "c": true
                }
              }
            }"#]]
        .assert_eq(&json);
        let deser_values: VecEnumValues = serde_json::from_str(&json).unwrap();
        assert_eq!(values, deser_values);
    }

    #[test]
    fn ron_serialize() {
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                EnumValue::Int(456),
                EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                EnumValue::Tuple(1, "Middle".to_string(), false),
                EnumValue::Struct {
                    a: 666,
                    b: "BBB".to_string(),
                    c: true,
                },
            ],
        };

        let ron = ron::ser::to_string_pretty(&values, ron::ser::PrettyConfig::new()).unwrap();
        expect_test::expect![[r#"
            (
                vec: {
                    "Int": 123,
                    "String": "FooBar",
                    "Int": 456,
                    "String": "XXX",
                    "Unit": (),
                    "Tuple": (1, "Middle", false),
                    "Struct": (
                        a: 666,
                        b: "BBB",
                        c: true,
                    ),
                },
            )"#]]
        .assert_eq(&ron);
        // TODO deserializing a Strings as an Identifier seems unsupported
        let deser_values: ron::Value = ron::de::from_str(&ron).unwrap();
        expect_test::expect![[r#"
            Map(
                Map(
                    {
                        String(
                            "vec",
                        ): Map(
                            Map(
                                {
                                    String(
                                        "Int",
                                    ): Number(
                                        Integer(
                                            456,
                                        ),
                                    ),
                                    String(
                                        "String",
                                    ): String(
                                        "XXX",
                                    ),
                                    String(
                                        "Struct",
                                    ): Map(
                                        Map(
                                            {
                                                String(
                                                    "a",
                                                ): Number(
                                                    Integer(
                                                        666,
                                                    ),
                                                ),
                                                String(
                                                    "b",
                                                ): String(
                                                    "BBB",
                                                ),
                                                String(
                                                    "c",
                                                ): Bool(
                                                    true,
                                                ),
                                            },
                                        ),
                                    ),
                                    String(
                                        "Tuple",
                                    ): Seq(
                                        [
                                            Number(
                                                Integer(
                                                    1,
                                                ),
                                            ),
                                            String(
                                                "Middle",
                                            ),
                                            Bool(
                                                false,
                                            ),
                                        ],
                                    ),
                                    String(
                                        "Unit",
                                    ): Unit,
                                },
                            ),
                        ),
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&deser_values);
    }

    #[test]
    fn xml_round_trip() {
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                EnumValue::Int(456),
                EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                // serialize_tuple and variants are not supported by XML
                // EnumValue::Tuple(1, "Middle".to_string(), false),
                // Cannot be deserialized. It serializes to:
                // <Struct><EnumValue><a>666</a><b>BBB</b><c>true</c></EnumValue></Struct>
                // EnumValue::Struct {
                //     a: 666,
                //     b: "BBB".to_string(),
                //     c: true,
                // },
            ],
        };

        let xml = serde_xml_rs::to_string(&values).unwrap();
        expect_test::expect![[r#"<VecEnumValues><vec><Int>123</Int><String>FooBar</String><Int>456</Int><String>XXX</String><Unit></Unit></vec></VecEnumValues>"#]]
        .assert_eq(&xml);
        let deser_values: VecEnumValues = serde_xml_rs::from_str(&xml).unwrap();
        assert_eq!(values, deser_values);
    }

    #[test]
    fn serde_test_round_trip() {
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                EnumValue::Int(456),
                EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                EnumValue::Tuple(1, "Middle".to_string(), false),
                EnumValue::Struct {
                    a: 666,
                    b: "BBB".to_string(),
                    c: true,
                },
            ],
        };

        use serde_test::Token::*;
        serde_test::assert_tokens(
            &values.readable(),
            &[
                Struct {
                    name: "VecEnumValues",
                    len: 1,
                },
                Str("vec"),
                Map {
                    len: Option::Some(7),
                },
                Str("Int"),
                I32(123),
                Str("String"),
                Str("FooBar"),
                Str("Int"),
                I32(456),
                Str("String"),
                Str("XXX"),
                Str("Unit"),
                Unit,
                Str("Tuple"),
                TupleStruct {
                    name: "EnumValue",
                    len: 3,
                },
                I32(1),
                Str("Middle"),
                Bool(false),
                TupleStructEnd,
                Str("Struct"),
                Struct {
                    name: "EnumValue",
                    len: 3,
                },
                Str("a"),
                I32(666),
                Str("b"),
                Str("BBB"),
                Str("c"),
                Bool(true),
                StructEnd,
                MapEnd,
                StructEnd,
            ],
        );
    }

    #[test]
    fn serde_test_round_trip_human_readable() {
        let values = VecEnumValues {
            vec: vec![EnumValue::Ip(
                IpAddr::from_str("127.0.0.1").unwrap(),
                IpAddr::from_str("::7777:dead:beef").unwrap(),
            )],
        };

        use serde_test::Token::*;
        serde_test::assert_tokens(
            &values.clone().readable(),
            &[
                Struct {
                    name: "VecEnumValues",
                    len: 1,
                },
                Str("vec"),
                Map {
                    len: Option::Some(1),
                },
                Str("Ip"),
                TupleStruct {
                    name: "EnumValue",
                    len: 2,
                },
                Str("127.0.0.1"),
                Str("::7777:dead:beef"),
                TupleStructEnd,
                MapEnd,
                StructEnd,
            ],
        );

        serde_test::assert_tokens(
            &values.compact(),
            &[
                Struct {
                    name: "VecEnumValues",
                    len: 1,
                },
                Str("vec"),
                Map {
                    len: Option::Some(1),
                },
                Str("Ip"),
                TupleStruct {
                    name: "EnumValue",
                    len: 2,
                },
                NewtypeVariant {
                    name: "IpAddr",
                    variant: "V4",
                },
                Tuple { len: 4 },
                U8(127),
                U8(0),
                U8(0),
                U8(1),
                TupleEnd,
                NewtypeVariant {
                    name: "IpAddr",
                    variant: "V6",
                },
                Tuple { len: 16 },
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0),
                U8(0x77),
                U8(0x77),
                U8(0xde),
                U8(0xad),
                U8(0xbe),
                U8(0xef),
                TupleEnd,
                TupleStructEnd,
                MapEnd,
                StructEnd,
            ],
        );
    }

    // Bincode does not support Deserializer::deserialize_identifier
    // https://github.com/bincode-org/bincode/blob/e0ac3245162ba668ba04591897dd88ff5b3096b8/src/de/mod.rs#L442

    #[test]
    fn rmp_round_trip() {
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                EnumValue::Int(456),
                EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                EnumValue::Tuple(1, "Middle".to_string(), false),
                EnumValue::Struct {
                    a: 666,
                    b: "BBB".to_string(),
                    c: true,
                },
                EnumValue::Ip(
                    IpAddr::from_str("127.0.0.1").unwrap(),
                    IpAddr::from_str("::7777:dead:beef").unwrap(),
                ),
            ],
        };

        let rmp = rmp_serde::to_vec(&values).unwrap();
        expect_test::expect![[r#"\x91\x88\xa3Int{\xa6String\xa6FooBar\xa3Int\xcd\x01\xc8\xa6String\xa3XXX\xa4Unit\xc0\xa5Tuple\x93\x01\xa6Middle\xc2\xa6Struct\x93\xcd\x02\x9a\xa3BBB\xc3\xa2Ip\x92\x81\x00\x94\x7f\x00\x00\x01\x81\x01\xdc\x00\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00ww\xcc\xde\xcc\xad\xcc\xbe\xcc\xef"#]]
        .assert_eq(&bytes_debug_readable(&rmp));
        let deser_values: VecEnumValues = rmp_serde::from_read(&*rmp).unwrap();
        assert_eq!(values, deser_values);
    }

    #[test]
    fn yaml_round_trip() {
        // Duplicate enum variants do not work with YAML
        let values = VecEnumValues {
            vec: vec![
                EnumValue::Int(123),
                EnumValue::String("FooBar".to_string()),
                // EnumValue::Int(456),
                // EnumValue::String("XXX".to_string()),
                EnumValue::Unit,
                EnumValue::Tuple(1, "Middle".to_string(), false),
                EnumValue::Struct {
                    a: 666,
                    b: "BBB".to_string(),
                    c: true,
                },
            ],
        };

        let yaml = serde_yaml::to_string(&values).unwrap();
        expect_test::expect![[r#"
            ---
            vec:
              Int: 123
              String: FooBar
              Unit: ~
              Tuple:
                - 1
                - Middle
                - false
              Struct:
                a: 666
                b: BBB
                c: true
        "#]]
        .assert_eq(&yaml);
        let deser_values: VecEnumValues = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(values, deser_values);
    }
}
