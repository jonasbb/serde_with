use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;

pub trait SerializeAs<T> {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub trait DeserializeAs<'de, T>: Sized {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>;

    // TODO: deserialize_as_into
}

impl SerializeAs<chrono::NaiveDateTime> for chrono::DateTime<chrono::Utc> {
    fn serialize_as<S>(source: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(*source, chrono::Utc);
        datetime.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, chrono::NaiveDateTime> for chrono::DateTime<chrono::Utc> {
    fn deserialize_as<D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        chrono::DateTime::<chrono::Utc>::deserialize(deserializer)
            .map(|datetime| datetime.naive_utc())
    }
}

pub struct Hex;

// TODO: AsRef
impl SerializeAs<Vec<u8>> for Hex {
    fn serialize_as<S>(source: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // FIXME: optimize
        serializer.serialize_str(&hex::encode(source))
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for Hex {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FIXME: map decode errors
        <&'de str as Deserialize<'de>>::deserialize(deserializer).map(|s| hex::decode(s).unwrap())
    }
}

// TODO: doc
pub struct SameAs<T>(PhantomData<T>);

impl<T: Serialize> SerializeAs<T> for SameAs<T> {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for SameAs<T> {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

impl<T, U> SerializeAs<Option<T>> for Option<U>
where
    U: SerializeAs<T>,
{
    fn serialize_as<S>(source: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *source {
            Some(ref value) => serializer.serialize_some(&SerializeAsWrap::<T, U>::new(value)),
            None => serializer.serialize_none(),
        }
    }
}

struct SerializeAsWrap<'a, T, U> {
    value: &'a T,
    marker: PhantomData<U>,
}

impl<'a, T, U> SerializeAsWrap<'a, T, U> {
    fn new(value: &'a T) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<'a, T, U> Serialize for SerializeAsWrap<'a, T, U>
where
    U: SerializeAs<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        U::serialize_as(self.value, serializer)
    }
}

macro_rules! seq_impl {
    ($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >) => {
        impl<T, U $(, $typaram)*> SerializeAs<$ty<T $(, $typaram)*>> for $ty<U $(, $typaram)*>
        where
            U: SerializeAs<T>,
            $(T: $tbound1 $(+ $tbound2)*,)*
            $($typaram: $bound,)*
        {
            fn serialize_as<S>(source: &$ty<T $(, $typaram)*>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_seq(source.iter().map(|item| SerializeAsWrap::<T, U>::new(item)))
            }
        }
    }
}

seq_impl!(BinaryHeap<T: Ord>);
seq_impl!(BTreeSet<T: Ord>);
seq_impl!(HashSet<T: Eq + Hash, H: BuildHasher>);
seq_impl!(LinkedList<T>);
seq_impl!(Vec<T>);
seq_impl!(VecDeque<T>);

struct OptionVisitor<T, U> {
    marker: PhantomData<T>,
    marker2: PhantomData<U>,
}

impl<'de, T, U> serde::de::Visitor<'de> for OptionVisitor<T, U>
where
    U: DeserializeAs<'de, T>,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("option")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        U::deserialize_as(deserializer).map(Some)
    }

    #[doc(hidden)]
    fn __private_visit_untagged_option<D>(self, deserializer: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Ok(U::deserialize_as(deserializer).ok())
    }
}

impl<'de, T, U> DeserializeAs<'de, Option<T>> for Option<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionVisitor::<T, U> {
            marker: PhantomData,
            marker2: PhantomData,
        })
    }
}

impl<'de, T, U> DeserializeAs<'de, Vec<T>> for Vec<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecVisitor<T, U> {
            marker: PhantomData<T>,
            marker2: PhantomData<U>,
        }

        impl<'de, T, U> serde::de::Visitor<'de> for VecVisitor<T, U>
        where
            U: DeserializeAs<'de, T>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values =
                    Vec::with_capacity(serde::private::de::size_hint::cautious(seq.size_hint()));

                while let Some(value) = seq
                    .next_element()?
                    .map(|v: DeserializeAsWrap<T, U>| v.into_inner())
                {
                    values.push(value);
                }

                Ok(values)
            }
        }

        let visitor = VecVisitor::<T, U> {
            marker: PhantomData,
            marker2: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

struct DeserializeAsWrap<T, U> {
    value: T,
    marker: PhantomData<U>,
}

impl<T, U> DeserializeAsWrap<T, U> {
    fn into_inner(self) -> T {
        self.value
    }
}

impl<'de, T, U> Deserialize<'de> for DeserializeAsWrap<T, U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        U::deserialize_as(deserializer).map(|value| Self {
            value,
            marker: PhantomData,
        })
    }
}

macro_rules! map_impl {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound:ident)* >) => {
        impl<K, KU, V, VU $(, $typaram)*> SerializeAs<$ty<K, V $(, $typaram)*>> for $ty<KU, VU $(, $typaram)*>
        where
            KU: SerializeAs<K>,
            VU: SerializeAs<V>,
            $(K: $kbound1 $(+ $kbound2)*,)*
            $($typaram: $bound,)*
        {
            fn serialize_as<S>(source: &$ty<K, V $(, $typaram)*>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_map(source.iter().map(|(k, v)| (SerializeAsWrap::<K, KU>::new(k), SerializeAsWrap::<V, VU>::new(v))))
            }
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);

macro_rules! map_impl2 {
    (
        $ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
        $access:ident,
        $with_capacity:expr
    ) => {
        impl<'de, K, V, KU, VU $(, $typaram)*> DeserializeAs<'de, $ty<K, V $(, $typaram)*>> for $ty<KU, VU $(, $typaram)*>
        where
            KU: DeserializeAs<'de, K>,
            VU: DeserializeAs<'de, V>,
            $(K: $kbound1 $(+ $kbound2)*,)*
            $($typaram: $bound1 $(+ $bound2)*),*
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$ty<K, V $(, $typaram)*>, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MapVisitor<K, V, KU, VU $(, $typaram)*> {
                    marker: PhantomData<$ty<K, V $(, $typaram)*>>,
                    marker2: PhantomData<$ty<KU, VU $(, $typaram)*>>,
                }

                impl<'de, K, V, KU, VU $(, $typaram)*> serde::de::Visitor<'de> for MapVisitor<K, V, KU, VU $(, $typaram)*>
                where
                        KU: DeserializeAs<'de, K>,
                        VU: DeserializeAs<'de, V>,
                        $(K: $kbound1 $(+ $kbound2)*,)*
                        $($typaram: $bound1 $(+ $bound2)*),*
                {
                    type Value = $ty<K, V $(, $typaram)*>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a map")
                    }

                    #[inline]
                    fn visit_map<A>(self, mut $access: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut values = $with_capacity;

                        while let Some((key, value)) = ($access.next_entry())?.map(|(k, v): (DeserializeAsWrap::<K, KU>, DeserializeAsWrap::<V, VU>)| (k.into_inner(), v.into_inner())) {
                            values.insert(key, value);
                        }

                        Ok(values)
                    }
                }

                let visitor = MapVisitor::<K, V, KU, VU $(, $typaram)*> { marker: PhantomData, marker2: PhantomData };
                deserializer.deserialize_map(visitor)
            }
        }
    }
}

map_impl2!(
    BTreeMap<K: Ord, V>,
    map,
    BTreeMap::new());

map_impl2!(
    HashMap<K: Eq + Hash, V, S: BuildHasher + Default>,
    map,
    HashMap::with_capacity_and_hasher(serde::private::de::size_hint::cautious(map.size_hint()), S::default()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrono() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<chrono::DateTime<chrono::Utc>>::serialize_as",
                deserialize_with = "<chrono::DateTime<chrono::Utc>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "chrono::DateTime<chrono::Utc>")]
            stamp: chrono::NaiveDateTime,
        }

        use std::str::FromStr;
        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_opt() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<Option<chrono::DateTime<chrono::Utc>>>::serialize_as",
                deserialize_with = "<Option<chrono::DateTime<chrono::Utc>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Option<chrono::DateTime<chrono::Utc>>")]
            stamp: Option<chrono::NaiveDateTime>,
        }

        use std::str::FromStr;
        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_opt_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<Vec<Option<chrono::DateTime<chrono::Utc>>>>::serialize_as",
                deserialize_with = "<Vec<Option<chrono::DateTime<chrono::Utc>>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Vec<Option<chrono::DateTime<chrono::Utc>>>")]
            stamps: Vec<Option<chrono::NaiveDateTime>>,
        }

        use std::str::FromStr;
        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: vec![
                    chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    chrono::NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
                ],
            })
            .unwrap(),
            "{\"stamps\":[\"1994-11-05T08:15:30Z\",\"1994-11-05T08:15:31Z\"]}"
        );

        assert_eq!(
            SomeTime {
                stamps: vec![
                    chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    chrono::NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
                ],
            },
            serde_json::from_str(
                "{\"stamps\":[\"1994-11-05T08:15:30Z\",\"1994-11-05T08:15:31Z\"]}"
            )
            .unwrap(),
        );
    }

    #[test]
    fn chrono_hash_map() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<HashMap<SameAs<i32>, chrono::DateTime<chrono::Utc>>>::serialize_as",
                deserialize_with = "<HashMap<SameAs<i32>, chrono::DateTime<chrono::Utc>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "HashMap<SameAs<i32>, chrono::DateTime<chrono::Utc>>")]
            stamps: HashMap<i32, chrono::NaiveDateTime>,
        }

        // FIXME: this test is flaky - random in hash-map sequence
        use std::str::FromStr;
        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: [
                    (
                        1,
                        chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
                    ),
                    (
                        2,
                        chrono::NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            })
            .unwrap(),
            "{\"stamps\":{\"1\":\"1994-11-05T08:15:30Z\",\"2\":\"1994-11-05T08:15:31Z\"}}"
        );

        assert_eq!(
            SomeTime {
                stamps: [
                    (
                        1,
                        chrono::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
                    ),
                    (
                        2,
                        chrono::NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            serde_json::from_str(
                "{\"stamps\":{\"1\":\"1994-11-05T08:15:30Z\",\"2\":\"1994-11-05T08:15:31Z\"}}"
            )
            .unwrap(),
        );
    }

    #[test]
    fn hex_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeBytes {
            #[serde(
                serialize_with = "<Vec<Hex>>::serialize_as",
                deserialize_with = "<Vec<Hex>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Vec<Hex>")]
            bytes: Vec<Vec<u8>>,
        }

        assert_eq!(
            serde_json::to_string(&SomeBytes {
                bytes: vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]]
            })
            .unwrap(),
            "{\"bytes\":[\"00010203\",\"04050607\"]}"
        );

        assert_eq!(
            SomeBytes {
                bytes: vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]]
            },
            serde_json::from_str("{\"bytes\":[\"00010203\",\"04050607\"]}").unwrap(),
        );
    }
}
