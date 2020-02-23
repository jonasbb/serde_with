use super::*;
use crate::serde_as::utils;
use serde::de::*;
use std::{
    collections::{BTreeMap, HashMap},
    convert::From,
    fmt::{self, Display},
    hash::{BuildHasher, Hash},
    str::FromStr,
};

impl<'de, T: Deserialize<'de>> DeserializeAs<'de, T> for SameAs<T> {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
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

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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

        impl<'de, T, U> Visitor<'de> for VecVisitor<T, U>
        where
            U: DeserializeAs<'de, T>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(utils::size_hint_cautious(seq.size_hint()));

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

pub(in crate::serde_as) struct DeserializeAsWrap<T, U> {
    value: T,
    marker: PhantomData<U>,
}

impl<T, U> DeserializeAsWrap<T, U> {
    pub(in crate::serde_as) fn into_inner(self) -> T {
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

                impl<'de, K, V, KU, VU $(, $typaram)*> Visitor<'de> for MapVisitor<K, V, KU, VU $(, $typaram)*>
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

impl<'de, T0, T1, As0, As1> DeserializeAs<'de, (T0, T1)> for (As0, As1)
where
    As0: DeserializeAs<'de, T0>,
    As1: DeserializeAs<'de, T1>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<(T0, T1), D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TupleVisitor<T0, T1>(PhantomData<(T0, T1)>);

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
            TupleVisitor::<DeserializeAsWrap<T0, As0>, DeserializeAsWrap<T1, As1>>(PhantomData),
        )
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

        struct SeqIter<'de, A, K, KAs, V, VAs> {
            access: A,
            marker: PhantomData<(&'de (), K, KAs, V, VAs)>,
        }

        impl<'de, A, K, KAs, V, VAs> Iterator for SeqIter<'de, A, K, KAs, V, VAs>
        where
            A: SeqAccess<'de>,
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
        {
            #[allow(clippy::type_complexity)]
            type Item = Result<(DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>), A::Error>;

            fn next(&mut self) -> Option<Self::Item> {
                self.access.next_element().transpose()
            }
        }

        impl<'de, K, KAs, V, VAs> Visitor<'de> for SeqVisitor<K, KAs, V, VAs>
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
            fn visit_seq<A>(self, access: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let iter = SeqIter {
                    access,
                    marker: PhantomData,
                };
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

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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
