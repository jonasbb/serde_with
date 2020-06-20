use super::*;
use crate::utils;
use rust::StringWithSeparator;
use serde::de::*;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    convert::From,
    fmt::{self, Display},
    hash::{BuildHasher, Hash},
    iter::FromIterator,
    str::FromStr,
    time::Duration,
};

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for SameAs<T> {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

impl<'de, T, U> DeserializeAs<'de, Box<T>> for Box<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Box<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Box::new(
            DeserializeAsWrap::<T, U>::deserialize(deserializer)?.into_inner(),
        ))
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
        struct OptionVisitor<T, U>(PhantomData<(T, U)>);

        impl<'de, T, U> Visitor<'de> for OptionVisitor<T, U>
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
                E: Error,
            {
                Ok(None)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
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

        deserializer.deserialize_option(OptionVisitor::<T, U>(PhantomData))
    }
}

macro_rules! seq_impl {
    (
        $ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* >,
        $access:ident,
        $with_capacity:expr,
        $append:ident
    ) => {
        impl<'de, T, U> DeserializeAs<'de, $ty<T>> for $ty<U>
        where
            U: DeserializeAs<'de, T>,
            $(T: $tbound1 $(+ $tbound2)*,)*
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$ty<T>, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct SeqVisitor<T, U> {
                    marker: PhantomData<T>,
                    marker2: PhantomData<U>,
                }

                impl<'de, T, U> Visitor<'de> for SeqVisitor<T, U>
                where
                    U: DeserializeAs<'de, T>,
                    $(T: $tbound1 $(+ $tbound2)*,)*
                {
                    type Value = $ty<T>;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    fn visit_seq<A>(self, mut $access: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut values = $with_capacity;

                        while let Some(value) = $access
                            .next_element()?
                            .map(|v: DeserializeAsWrap<T, U>| v.into_inner())
                        {
                            values.$append(value);
                        }

                        Ok(values.into())
                    }
                }

                let visitor = SeqVisitor::<T, U> {
                    marker: PhantomData,
                    marker2: PhantomData,
                };
                deserializer.deserialize_seq(visitor)
            }
        }
    };
}

type BoxedSlice<T> = Box<[T]>;
seq_impl!(
    BinaryHeap<T: Ord>,
    seq,
    BinaryHeap::with_capacity(utils::size_hint_cautious(seq.size_hint())),
    push
);
seq_impl!(
    BoxedSlice<T>,
    seq,
    Vec::with_capacity(utils::size_hint_cautious(seq.size_hint())),
    push
);
seq_impl!(BTreeSet<T: Ord>, seq, BTreeSet::new(), insert);
seq_impl!(
    HashSet<T: Eq + Hash>,
    seq,
    HashSet::with_capacity(utils::size_hint_cautious(seq.size_hint())),
    insert
);
seq_impl!(LinkedList<T>, seq, LinkedList::new(), push_back);
seq_impl!(
    Vec<T>,
    seq,
    Vec::with_capacity(utils::size_hint_cautious(seq.size_hint())),
    push
);
seq_impl!(
    VecDeque<T>,
    seq,
    VecDeque::with_capacity(utils::size_hint_cautious(seq.size_hint())),
    push_back
);

macro_rules! map_impl2 {
    (
        $ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
        // We need an external name, such that we can use it in the `with_capacity` expression
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

                impl<'de, K, V, KU, VU $(, $typaram)*> Visitor<'de> for MapVisitor<K, V, KU, VU $(, $typaram)*>
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
                        A: MapAccess<'de>,
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
    HashMap::with_capacity_and_hasher(utils::size_hint_cautious(map.size_hint()), S::default()));

impl<'de, T> DeserializeAs<'de, T> for DisplayFromStr
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

macro_rules! tuple_impl {
    ($len:literal $($n:tt $t:ident $tas:ident)+) => {
        impl<'de, $($t, $tas,)+> DeserializeAs<'de, ($($t,)+)> for ($($tas,)+)
        where
            $($tas: DeserializeAs<'de, $t>,)+
        {
            fn deserialize_as<D>(deserializer: D) -> Result<($($t,)+), D::Error>
            where
                D: Deserializer<'de>,
            {
                struct TupleVisitor<$($t,)+>(PhantomData<($($t,)+)>);

                impl<'de, $($t, $tas,)+> Visitor<'de>
                    for TupleVisitor<$(DeserializeAsWrap<$t, $tas>,)+>
                where
                    $($tas: DeserializeAs<'de, $t>,)+
                {
                    type Value = ($($t,)+);

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str(concat!("a tuple of size ", $len))
                    }

                    #[allow(non_snake_case)]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        $(
                            let $t: DeserializeAsWrap<$t, $tas> = match seq.next_element()? {
                                Some(value) => value,
                                None => return Err(Error::invalid_length($n, &self)),
                            };
                        )+

                        Ok(($($t.into_inner(),)+))
                    }
                };

                deserializer.deserialize_tuple(
                    $len,
                    TupleVisitor::<$(DeserializeAsWrap<$t, $tas>,)+>(PhantomData),
                )
            }
        }
    };
}

tuple_impl!(1 0 T0 As0);
tuple_impl!(2 0 T0 As0 1 T1 As1);
tuple_impl!(3 0 T0 As0 1 T1 As1 2 T2 As2);
tuple_impl!(4 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3);
tuple_impl!(5 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4);
tuple_impl!(6 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5);
tuple_impl!(7 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6);
tuple_impl!(8 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7);
tuple_impl!(9 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8);
tuple_impl!(10 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9);
tuple_impl!(11 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10);
tuple_impl!(12 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10 11 T11 As11);
tuple_impl!(13 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10 11 T11 As11 12 T12 As12);
tuple_impl!(14 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10 11 T11 As11 12 T12 As12 13 T13 As13);
tuple_impl!(15 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10 11 T11 As11 12 T12 As12 13 T13 As13 14 T14 As14);
tuple_impl!(16 0 T0 As0 1 T1 As1 2 T2 As2 3 T3 As3 4 T4 As4 5 T5 As5 6 T6 As6 7 T7 As7 8 T8 As8 9 T9 As9 10 T10 As10 11 T11 As11 12 T12 As12 13 T13 As13 14 T14 As14 15 T15 As15);

impl<'de, T, As> DeserializeAs<'de, [T; 0]> for [As; 0] {
    fn deserialize_as<D>(deserializer: D) -> Result<[T; 0], D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor<T>(PhantomData<T>);

        impl<'de, T, As> Visitor<'de> for ArrayVisitor<DeserializeAsWrap<T, As>> {
            type Value = [T; 0];

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!("an array of size ", 0))
            }

            #[allow(non_snake_case)]
            fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok([])
            }
        };

        deserializer.deserialize_tuple(0, ArrayVisitor::<DeserializeAsWrap<T, As>>(PhantomData))
    }
}

macro_rules! array_impl {
    ($len:literal $($idx:tt)*) => {
        impl<'de, T, As> DeserializeAs<'de, [T; $len]> for [As; $len]
        where
            As: DeserializeAs<'de, T>,
        {
            fn deserialize_as<D>(deserializer: D) -> Result<[T; $len], D::Error>
            where
                D: Deserializer<'de>,
            {
                struct ArrayVisitor<T>(PhantomData<T>);

                impl<'de, T, As> Visitor<'de>
                    for ArrayVisitor<DeserializeAsWrap<T, As>>
                where
                    As: DeserializeAs<'de, T>,
                {
                    type Value = [T; $len];

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str(concat!("an array of size ", $len))
                    }

                    #[allow(non_snake_case)]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        Ok([$(
                            match seq.next_element::<DeserializeAsWrap<T, As>>()? {
                                Some(value) => value.into_inner(),
                                None => return Err(Error::invalid_length($idx, &self)),
                            },
                        )*])
                    }
                };

                deserializer.deserialize_tuple(
                    $len,
                    ArrayVisitor::<DeserializeAsWrap<T, As>>(PhantomData),
                )
            }
        }
    };
}

array_impl!(1 0);
array_impl!(2 0 1);
array_impl!(3 0 1 2);
array_impl!(4 0 1 2 3);
array_impl!(5 0 1 2 3 4);
array_impl!(6 0 1 2 3 4 5);
array_impl!(7 0 1 2 3 4 5 6);
array_impl!(8 0 1 2 3 4 5 6 7);
array_impl!(9 0 1 2 3 4 5 6 7 8);
array_impl!(10 0 1 2 3 4 5 6 7 8 9);
array_impl!(11 0 1 2 3 4 5 6 7 8 9 10);
array_impl!(12 0 1 2 3 4 5 6 7 8 9 10 11);
array_impl!(13 0 1 2 3 4 5 6 7 8 9 10 11 12);
array_impl!(14 0 1 2 3 4 5 6 7 8 9 10 11 12 13);
array_impl!(15 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
array_impl!(16 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
array_impl!(17 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16);
array_impl!(18 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17);
array_impl!(19 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18);
array_impl!(20 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19);
array_impl!(21 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20);
array_impl!(22 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21);
array_impl!(23 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22);
array_impl!(24 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23);
array_impl!(25 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24);
array_impl!(26 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25);
array_impl!(27 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26);
array_impl!(28 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27);
array_impl!(29 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28);
array_impl!(30 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29);
array_impl!(31 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30);
array_impl!(32 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for Same {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

macro_rules! map_as_tuple_seq {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V>) => {
        impl<'de, K, KAs, V, VAs> DeserializeAs<'de, $ty<K, V>> for Vec<(KAs, VAs)>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
            $(K: $kbound1 $(+ $kbound2)*,)*
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$ty<K, V>, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct SeqVisitor<K, KAs, V, VAs> {
                    marker: PhantomData<(K, KAs, V, VAs)>,
                }

                impl<'de, K, KAs, V, VAs> Visitor<'de> for SeqVisitor<K, KAs, V, VAs>
                where
                    KAs: DeserializeAs<'de, K>,
                    VAs: DeserializeAs<'de, V>,
                    $(K: $kbound1 $(+ $kbound2)*,)*
                {
                    type Value = $ty<K, V>;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    #[inline]
                    fn visit_seq<A>(self, access: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let iter = utils::SeqIter::new(access);
                        iter.map(|res| {
                            res.map(
                                |(k, v): (DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>)| {
                                    (k.into_inner(), v.into_inner())
                                },
                            )
                        })
                        .collect()
                    }
                }

                let visitor = SeqVisitor::<K, KAs, V, VAs> {
                    marker: PhantomData,
                };
                deserializer.deserialize_seq(visitor)
            }
        }
    };
}
map_as_tuple_seq!(BTreeMap<K: Ord, V>);
map_as_tuple_seq!(HashMap<K: Eq + Hash, V>);

impl<'de, Str> DeserializeAs<'de, Option<Str>> for NoneAsEmptyString
where
    Str: for<'a> From<&'a str>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Str>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionStringEmptyNone<Str>(PhantomData<Str>);
        impl<'de, Str> Visitor<'de> for OptionStringEmptyNone<Str>
        where
            Str: for<'a> From<&'a str>,
        {
            type Value = Option<Str>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(match value {
                    "" => None,
                    v => Some(Str::from(v)),
                })
            }
        }

        deserializer.deserialize_str(OptionStringEmptyNone(PhantomData))
    }
}

macro_rules! tuple_seq_as_map_impl_intern {
    ($tyorig:ident < (K $(: $($kbound:ident $(+)?)+)?, V $(: $($vbound:ident $(+)?)+)?)>, $ty:ident <KAs, VAs>) => {
        #[allow(clippy::implicit_hasher)]
        impl<'de, K, KAs, V, VAs> DeserializeAs<'de, $tyorig < (K, V) >> for $ty<KAs, VAs>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
            K: $($($kbound +)*)*,
            V: $($($vbound +)*)*,
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$tyorig < (K, V) >, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MapVisitor<K, KAs, V, VAs> {
                    marker: PhantomData<(K, KAs, V, VAs)>,
                }

                impl<'de, K, KAs, V, VAs> Visitor<'de> for MapVisitor<K, KAs, V, VAs>
                where
                    KAs: DeserializeAs<'de, K>,
                    VAs: DeserializeAs<'de, V>,
                    K: $($($kbound +)*)*,
                    V: $($($vbound +)*)*,
                {
                    type Value = $tyorig < (K, V) >;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("a map")
                    }

                    #[inline]
                    fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let iter = utils::MapIter::new(access);
                        iter.map(|res| {
                            res.map(
                                |(k, v): (DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>)| {
                                    (k.into_inner(), v.into_inner())
                                },
                            )
                        })
                        .collect()
                    }
                }

                let visitor = MapVisitor::<K, KAs, V, VAs> {
                    marker: PhantomData,
                };
                deserializer.deserialize_map(visitor)
            }
        }
    }
}
macro_rules! tuple_seq_as_map_impl {
    ($($tyorig:ident < (K $(: $($kbound:ident $(+)?)+)?, V $(: $($vbound:ident $(+)?)+)?)> $(,)?)+) => {$(
        tuple_seq_as_map_impl_intern!($tyorig < (K $(: $($kbound +)+)?, V $(: $($vbound +)+)?) >, BTreeMap<KAs, VAs>);
        tuple_seq_as_map_impl_intern!($tyorig < (K $(: $($kbound +)+)?, V $(: $($vbound +)+)?) >, HashMap<KAs, VAs>);
    )+}
}

tuple_seq_as_map_impl! {
    BinaryHeap<(K: Ord, V: Ord)>,
    BTreeSet<(K: Ord, V: Ord)>,
    HashSet<(K: Eq + Hash, V: Eq + Hash)>,
    LinkedList<(K, V)>,
    Vec<(K, V)>,
    VecDeque<(K, V)>,
}

macro_rules! tuple_seq_as_map_option_impl {
    ($($ty:ident $(,)?)+) => {$(
        #[allow(clippy::implicit_hasher)]
        impl<'de, K, KAs, V, VAs> DeserializeAs<'de, Option<(K, V)>> for $ty<KAs, VAs>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
        {
            fn deserialize_as<D>(deserializer: D) -> Result<Option<(K, V)>, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MapVisitor<K, KAs, V, VAs> {
                    marker: PhantomData<(K, KAs, V, VAs)>,
                }

                impl<'de, K, KAs, V, VAs> Visitor<'de> for MapVisitor<K, KAs, V, VAs>
                where
                    KAs: DeserializeAs<'de, K>,
                    VAs: DeserializeAs<'de, V>,
                {
                    type Value = Option<(K, V)>;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("a map of size 1")
                    }

                    #[inline]
                    fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let iter = utils::MapIter::new(access);
                        iter.map(|res| {
                            res.map(
                                |(k, v): (DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>)| {
                                    (k.into_inner(), v.into_inner())
                                },
                            )
                        })
                        .next()
                        .transpose()
                    }
                }

                let visitor = MapVisitor::<K, KAs, V, VAs> {
                    marker: PhantomData,
                };
                deserializer.deserialize_map(visitor)
            }
        }
    )+}
}
tuple_seq_as_map_option_impl!(BTreeMap, HashMap);

impl<'de, T, TAs> DeserializeAs<'de, T> for DefaultOnError<TAs>
where
    TAs: DeserializeAs<'de, T>,
    T: Default,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        TAs::deserialize_as(deserializer).or_else(|_| Ok(Default::default()))
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for BytesOrString {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::rust::bytes_or_string::deserialize(deserializer)
    }
}

struct DurationVisitiorFlexible;
impl<'de> Visitor<'de> for DurationVisitiorFlexible {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("an integer, a float, or a string containing a number")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value >= 0 {
            Ok(Duration::new(value as u64, 0))
        } else {
            Err(Error::custom(format!(
                "Negative values are not supported for Duration. Found {}",
                value
            )))
        }
    }

    fn visit_u64<E>(self, secs: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Duration::new(secs, 0))
    }

    fn visit_f64<E>(self, secs: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        utils::duration_from_secs_f64(secs).map_err(Error::custom)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let parts: Vec<_> = value.split('.').collect();

        match *parts.as_slice() {
            [seconds] => {
                if let Ok(seconds) = u64::from_str_radix(seconds, 10) {
                    Ok(Duration::new(seconds, 0))
                } else {
                    Err(Error::invalid_value(Unexpected::Str(value), &self))
                }
            }
            [seconds, subseconds] => {
                if let Ok(seconds) = u64::from_str_radix(seconds, 10) {
                    let subseclen = subseconds.chars().count() as u32;
                    if subseclen > 9 {
                        return Err(Error::custom(format!(
                                    "Duration only support nanosecond precision but '{}' has more than 9 digits.",
                                    value
                                )));
                    }

                    if let Ok(mut subseconds) = u32::from_str_radix(subseconds, 10) {
                        // convert subseconds to nanoseconds (10^-9), require 9 places for nanoseconds
                        subseconds *= 10u32.pow(9 - subseclen);
                        Ok(Duration::new(seconds, subseconds))
                    } else {
                        Err(Error::invalid_value(Unexpected::Str(value), &self))
                    }
                } else {
                    Err(Error::invalid_value(Unexpected::Str(value), &self))
                }
            }

            _ => Err(Error::invalid_value(Unexpected::Str(value), &self)),
        }
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<Integer, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::deserialize(deserializer).map(|secs| Duration::new(secs, 0))
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        utils::duration_from_secs_f64(val).map_err(Error::custom)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSeconds<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::rust::display_fromstr::deserialize(deserializer).map(|secs| Duration::new(secs, 0))
    }
}

impl<'de, FORMAT> DeserializeAs<'de, Duration> for DurationSeconds<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitiorFlexible)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<f64, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        utils::duration_from_secs_f64(val).map_err(Error::custom)
    }
}

impl<'de> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<String, Strict> {
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dur = String::deserialize(deserializer)?;
        DurationVisitiorFlexible.visit_str(&*dur)
    }
}

impl<'de, FORMAT> DeserializeAs<'de, Duration> for DurationSecondsWithFrac<FORMAT, Flexible>
where
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitiorFlexible)
    }
}

impl<'de, SEPARATOR, I, T> DeserializeAs<'de, I> for StringWithSeparator<SEPARATOR, T>
where
    SEPARATOR: Separator,
    I: FromIterator<T>,
    T: FromStr,
    T::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<I, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None.into_iter().collect())
        } else {
            s.split(SEPARATOR::separator())
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()
                .map_err(Error::custom)
        }
    }
}
