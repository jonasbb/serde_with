#![allow(missing_debug_implementations, missing_docs)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt::{self, Debug, Display},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    str::FromStr,
};

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "hex")]
mod hex;

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

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(size_hint_cautious(seq.size_hint()));

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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    HashMap::with_capacity_and_hasher(size_hint_cautious(map.size_hint()), S::default()));

pub struct As<T>(PhantomData<T>);

impl<T> As<T> {
    pub fn serialize<S, I>(value: &I, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: SerializeAs<I>,
    {
        T::serialize_as(value, serializer)
    }

    pub fn deserialize<'de, D, I>(deserializer: D) -> Result<I, D::Error>
    where
        T: DeserializeAs<'de, I>,
        D: Deserializer<'de>,
    {
        T::deserialize_as(deserializer)
    }
}

/// Re-Implementation of `serde::private::de::size_hint::cautious`
#[inline]
fn size_hint_cautious(hint: Option<usize>) -> usize {
    std::cmp::min(hint.unwrap_or(0), 4096)
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DisplayString;

impl<T> SerializeAs<T> for DisplayString
where
    T: Display,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        crate::rust::display_fromstr::serialize(source, serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for DisplayString
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::rust::display_fromstr::deserialize(deserializer)
    }
}

impl<T0, T1, As0, As1> SerializeAs<(T0, T1)> for (As0, As1)
where
    As0: SerializeAs<T0>,
    As1: SerializeAs<T1>,
{
    fn serialize_as<S>((elem0, elem1): &(T0, T1), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&SerializeAsWrap::<T0, As0>::new(elem0))?;
        tup.serialize_element(&SerializeAsWrap::<T1, As1>::new(elem1))?;
        tup.end()
    }
}

impl<'de, T0, T1, As0, As1> DeserializeAs<'de, (T0, T1)> for (As0, As1)
where
    As0: DeserializeAs<'de, T0>,
    As1: DeserializeAs<'de, T1>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<(T0, T1), D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};

        struct TupleVisitor<T0, T1> {
            marker: PhantomData<(T0, T1)>,
        }

        impl<'de, T0, As0, T1, As1> Visitor<'de>
            for TupleVisitor<DeserializeAsWrap<T0, As0>, DeserializeAsWrap<T1, As1>>
        where
            As0: DeserializeAs<'de, T0>,
            As1: DeserializeAs<'de, T1>,
        {
            type Value = (T0, T1);

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a tuple of size 2")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let t0: DeserializeAsWrap<T0, As0> = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(Error::invalid_length(0, &self)),
                };
                let t1: DeserializeAsWrap<T1, As1> = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(Error::invalid_length(1, &self)),
                };

                Ok((t0.into_inner(), t1.into_inner()))
            }
        };

        deserializer.deserialize_tuple(
            2,
            TupleVisitor::<DeserializeAsWrap<T0, As0>, DeserializeAsWrap<T1, As1>> {
                marker: PhantomData,
            },
        )
    }
}

// impl<K, KAs, V, VAs> SerializeAs<Vec<(KAs, VAs)>> for BTreeMap<K, V>
// where
//     K: SerializeAs<KAs>,
//     V: SerializeAs<VAs>,
// {
//     fn serialize_as<S>(source: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.collect_seq(source.iter().map(|(key, value)| SerializeAsWrap::<T, U>::new(item)))
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use serde::de::DeserializeOwned;

    fn is_equal<T>(value: T, s: &str)
    where
        T: Debug + DeserializeOwned + PartialEq + Serialize,
    {
        assert_eq!(
            serde_json::from_str::<T>(s).unwrap(),
            value,
            "Deserialization differs from expected value."
        );
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            s,
            "Serialization differs from expected value."
        );
    }

    #[test]
    fn test_display_fromstr() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct {
            #[serde(with = "As::<DisplayString>")]
            value: u32,
        };

        is_equal(Struct { value: 123 }, r#"{"value":"123"}"#);
    }

    #[test]
    fn test_tuples() {
        use std::net::IpAddr;
        let ip = "1.2.3.4".parse().unwrap();

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct {
            #[serde(with = "As::<(DisplayString, DisplayString)>")]
            values: (u32, IpAddr),
        };
        is_equal(
            Struct {
                values: (555_888, ip),
            },
            r#"{"values":["555888","1.2.3.4"]}"#,
        );

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct2 {
            #[serde(with = "As::<(SameAs<u32>, DisplayString)>")]
            values: (u32, IpAddr),
        };
        is_equal(
            Struct2 { values: (987, ip) },
            r#"{"values":[987,"1.2.3.4"]}"#,
        );

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct3 {
            #[serde(with = "As::<(Same, DisplayString)>")]
            values: (u32, IpAddr),
        };
        is_equal(
            Struct3 { values: (987, ip) },
            r#"{"values":[987,"1.2.3.4"]}"#,
        );
    }

    #[test]
    fn test_map_as_tuple_list() {
        use std::net::IpAddr;
        let ip = "1.2.3.4".parse().unwrap();
        let ip2 = "255.255.255.255".parse().unwrap();

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct {
            #[serde(with = "As::<Vec<(DisplayString, DisplayString)>>")]
            values: BTreeMap<u32, IpAddr>,
        };

        let map: BTreeMap<_, _> = vec![(1, ip), (10, ip), (200, ip2)].into_iter().collect();
        is_equal(
            Struct {
                values: map.clone(),
            },
            r#"{"values":[["1","1.2.3.4"],["10","1.2.3.4"],["200","255.255.255.255"]]}"#,
        );

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Struct2 {
            #[serde(with = "As::<Vec<(Same, DisplayString)>>")]
            values: BTreeMap<u32, IpAddr>,
        };

        is_equal(
            Struct2 { values: map },
            r#"{"values":[[1,"1.2.3.4"],[10,"1.2.3.4"],[200,"255.255.255.255"]]}"#,
        );
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Same;

impl<T: Serialize> SerializeAs<T> for Same {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for Same {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

impl<K, KAs, V, VAs> SerializeAs<BTreeMap<K, V>> for Vec<(KAs, VAs)>
where
    KAs: SerializeAs<K>,
    VAs: SerializeAs<V>,
{
    fn serialize_as<S>(source: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(source.iter().map(|(k, v)| {
            (
                SerializeAsWrap::<K, KAs>::new(k),
                SerializeAsWrap::<V, VAs>::new(v),
            )
        }))
    }
}

impl<'de, K, KAs, V, VAs> DeserializeAs<'de, BTreeMap<K, V>> for Vec<(KAs, VAs)>
where
    KAs: DeserializeAs<'de, K>,
    VAs: DeserializeAs<'de, V>,
    K: Ord,
{
    fn deserialize_as<D>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeqVisitor<K, KAs, V, VAs> {
            marker: PhantomData<(K, KAs, V, VAs)>,
        }

        impl<'de, K, KAs, V, VAs> serde::de::Visitor<'de> for SeqVisitor<K, KAs, V, VAs>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
            K: Ord,
        {
            type Value = BTreeMap<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = BTreeMap::new();

                while let Some((key, value)) = (access.next_element())?.map(
                    |(k, v): (DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>)| {
                        (k.into_inner(), v.into_inner())
                    },
                ) {
                    values.insert(key, value);
                }

                Ok(values)
            }
        }

        let visitor = SeqVisitor::<K, KAs, V, VAs> {
            marker: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}
