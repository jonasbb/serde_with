use super::*;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Display,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

impl<T: Serialize> SerializeAs<T> for SameAs<T> {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(serializer)
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

pub(in crate::serde_as) struct SerializeAsWrap<'a, T, U> {
    value: &'a T,
    marker: PhantomData<U>,
}

impl<'a, T, U> SerializeAsWrap<'a, T, U> {
    pub(in crate::serde_as) fn new(value: &'a T) -> Self {
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

impl<T: Serialize> SerializeAs<T> for Same {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(serializer)
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
