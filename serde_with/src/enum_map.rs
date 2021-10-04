//! TODO
//!
//! Could fix this:
//! https://github.com/serde-rs/json/issues/743
#![allow(unused_variables)]

use crate::content::ser::{Content, ContentSerializer};
use serde::de::{DeserializeSeed, EnumAccess, Error, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::ser::{
    Impossible, SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTupleVariant,
};
use serde::{ser, Deserializer, Serialize, Serializer};

static END_OF_MAP_IDENTIFIER: &str = "__PRIVATE_END_OF_MAP_MARKER__";

// Serialization code below here

struct MyVecSerializer<S>(S);

impl<S> Serializer for MyVecSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;

    type Error = S::Error;

    type SerializeSeq = MySerializeSeq<S::SerializeMap>;

    type SerializeTuple = Impossible<S::Ok, S::Error>;

    type SerializeTupleStruct = Impossible<S::Ok, S::Error>;

    type SerializeTupleVariant = Impossible<S::Ok, S::Error>;

    type SerializeMap = Impossible<S::Ok, S::Error>;

    type SerializeStruct = Impossible<S::Ok, S::Error>;

    type SerializeStructVariant = Impossible<S::Ok, S::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let is_human_readable = self.0.is_human_readable();
        self.0.serialize_map(len).map(|delegate| MySerializeSeq {
            delegate,
            is_human_readable,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }
}

/// Serialize a single element but turn the sequence into a map logic.
///
/// It uses [`SerializeEnumAsMapElement`] for the map element serialization.
///
/// The [`Serializer`] implementation handles all the `serialize_*_variant` functions and defers to [`SerializeVariant`] for the more complicated tuple and struct variants.
struct MySerializeSeq<M> {
    delegate: M,
    is_human_readable: bool,
}

impl<M> SerializeSeq for MySerializeSeq<M>
where
    M: SerializeMap,
{
    type Ok = M::Ok;

    type Error = M::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(MySerializeMapSerializer {
            delegate: &mut self.delegate,
            is_human_readable: self.is_human_readable,
        })?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.delegate.end()
    }
}

struct MySerializeMapSerializer<'a, M> {
    delegate: &'a mut M,
    is_human_readable: bool,
}

impl<'a, M> Serializer for MySerializeMapSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();

    type Error = M::Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    type SerializeTuple = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = MySerializeVariant<'a, M>;

    type SerializeMap = Impossible<Self::Ok, Self::Error>;

    type SerializeStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeStructVariant = MySerializeVariant<'a, M>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.delegate.serialize_entry(variant, &())?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.delegate.serialize_entry(variant, value)?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(MySerializeVariant {
            delegate: self.delegate,
            is_human_readable: self.is_human_readable,
            variant,
            content: Content::TupleStruct(name, Vec::with_capacity(len)),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("wrong type for VecEnumMap"))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(MySerializeVariant {
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
struct MySerializeVariant<'a, M> {
    delegate: &'a mut M,
    is_human_readable: bool,
    variant: &'static str,
    content: Content,
}

impl<'a, M> SerializeStructVariant for MySerializeVariant<'a, M>
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

impl<'a, M> SerializeTupleVariant for MySerializeVariant<'a, M>
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

struct MyVecDeserializer<M>(M);

impl<'de, M> Deserializer<'de> for MyVecDeserializer<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(MySeqAccess(self.0))
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

struct MySeqAccess<M>(M);

impl<'de, M> SeqAccess<'de> for MySeqAccess<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match seed.deserialize(MyMapAccessDeserializer(&mut self.0)) {
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

struct MyMapAccessDeserializer<M>(M);

impl<'de, M> Deserializer<'de> for MyMapAccessDeserializer<M>
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
        mut self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(MyEnumAccess(&mut self.0))
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(&mut self.0)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct struct identifier ignored_any
    }
}

struct MyEnumAccess<M>(M);

impl<'de, M> EnumAccess<'de> for MyEnumAccess<M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;
    type Variant = MapAsEnum<M>;

    fn variant_seed<T>(mut self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.next_key_seed(seed)? {
            Some(key) => Ok((key, MapAsEnum(self.0))),

            // Unfortunately we loose the optional aspect of MapAccess, so we need to special case an error value to mark the end of the map.
            None => Err(Error::custom(END_OF_MAP_IDENTIFIER)),
        }
    }
}

// https://docs.rs/serde/1.0.130/src/serde/de/value.rs.html#1532
struct MapAsEnum<M>(M);

impl<'de, M> VariantAccess<'de> for MapAsEnum<M>
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
    use std::fmt;
    use std::net::IpAddr;
    use std::str::FromStr;

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

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct VecEnumValues(Vec<EnumValue>);

    impl Serialize for VecEnumValues {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(MyVecSerializer(serializer))
        }
    }

    impl<'de> Deserialize<'de> for VecEnumValues {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> Visitor<'de> for MyVisitor {
                type Value = VecEnumValues;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, "TODO")
                }

                fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                    let deserializer = MyVecDeserializer(map);
                    let v = Vec::deserialize(deserializer)?;
                    Ok(VecEnumValues(v))
                }
            }

            deserializer.deserialize_map(MyVisitor)
        }
    }

    #[test]
    fn json_round_trip() {
        let values = VecEnumValues(vec![
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
        ]);

        let json = serde_json::to_string_pretty(&values).unwrap();
        expect_test::expect![[r#"
            {
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
            }"#]]
        .assert_eq(&json);
        let deser_values: VecEnumValues = serde_json::from_str(&json).unwrap();
        assert_eq!(values, deser_values);
    }

    #[test]
    fn ron_serialize() {
        let values = VecEnumValues(vec![
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
        ]);

        let ron = ron::ser::to_string_pretty(&values, ron::ser::PrettyConfig::new()).unwrap();
        expect_test::expect![[r#"
            {
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
            }"#]]
        .assert_eq(&ron);
        // TODO deserializing a Strings as an Identifier seems unsupported
        let deser_values: ron::Value = ron::de::from_str(&ron).unwrap();
        expect_test::expect![[r#"
            Map(
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
            )
        "#]]
        .assert_debug_eq(&deser_values);
    }

    #[test]
    fn xml_serialize() {
        let values = VecEnumValues(vec![
            EnumValue::Int(123),
            EnumValue::String("FooBar".to_string()),
            EnumValue::Int(456),
            EnumValue::String("XXX".to_string()),
            EnumValue::Unit,
            // serialize_tuple and variants are not supported by XML
            // EnumValue::Tuple(1, "Middle".to_string(), false),
            EnumValue::Struct {
                a: 666,
                b: "BBB".to_string(),
                c: true,
            },
        ]);

        let xml = serde_xml_rs::to_string(&values).unwrap();
        expect_test::expect![[r#"<Int>123</Int><String>FooBar</String><Int>456</Int><String>XXX</String><Unit></Unit><Struct><EnumValue><a>666</a><b>BBB</b><c>true</c></EnumValue></Struct>"#]]
        .assert_eq(&xml);
    }

    #[test]
    fn serde_test_round_trip() {
        let values = VecEnumValues(vec![
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
        ]);

        use serde_test::Token::*;
        serde_test::assert_tokens(
            &values.readable(),
            &[
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
            ],
        );
    }

    #[test]
    fn serde_test_round_trip_human_readable() {
        let values = VecEnumValues(vec![EnumValue::Ip(
            IpAddr::from_str("127.0.0.1").unwrap(),
            IpAddr::from_str("::7777:dead:beef").unwrap(),
        )]);

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
        let values = VecEnumValues(vec![
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
        ]);

        let rmp = rmp_serde::to_vec(&values).unwrap();
        expect_test::expect![[r#"[136, 163, 73, 110, 116, 123, 166, 83, 116, 114, 105, 110, 103, 166, 70, 111, 111, 66, 97, 114, 163, 73, 110, 116, 205, 1, 200, 166, 83, 116, 114, 105, 110, 103, 163, 88, 88, 88, 164, 85, 110, 105, 116, 192, 165, 84, 117, 112, 108, 101, 147, 1, 166, 77, 105, 100, 100, 108, 101, 194, 166, 83, 116, 114, 117, 99, 116, 147, 205, 2, 154, 163, 66, 66, 66, 195, 162, 73, 112, 146, 129, 0, 148, 127, 0, 0, 1, 129, 1, 220, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 119, 204, 222, 204, 173, 204, 190, 204, 239]"#]]
        .assert_eq(&format!("{:?}", rmp));
        let deser_values: VecEnumValues = rmp_serde::from_read(&*rmp).unwrap();
        assert_eq!(values, deser_values);
    }

    #[test]
    fn yaml_round_trip() {
        // Duplicate enum variants do not work with YAML
        let values = VecEnumValues(vec![
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
        ]);

        let yaml = serde_yaml::to_string(&values).unwrap();
        expect_test::expect![[r#"
            ---
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
