use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt,
    hash::Hash,
    marker::PhantomData,
};

pub trait PreventDuplicateInsertsSet<T> {
    fn new(size_hint: Option<usize>) -> Self;

    /// Return true if the insert was successful and the value did not exist in the set
    fn insert(&mut self, value: T) -> bool;
}

pub trait PreventDuplicateInsertsMap<K, V> {
    fn new(size_hint: Option<usize>) -> Self;

    /// Return true if the insert was successful and the key did not exist in the map
    fn insert(&mut self, key: K, value: V) -> bool;
}

/// Deserialize a set and return an error on duplicate values
pub fn deserialize_set<'de, D, T, V>(deserializer: D) -> Result<T, D::Error>
where
    T: PreventDuplicateInsertsSet<V>,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct SeqVisitor<T, V> {
        marker: PhantomData<T>,
        marker2: PhantomData<V>,
    };

    impl<'de, T, V> Visitor<'de> for SeqVisitor<T, V>
    where
        T: PreventDuplicateInsertsSet<V>,
        V: Deserialize<'de>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence")
        }

        #[inline]
        fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Self::Value::new(access.size_hint());

            while let Some(value) = access.next_element()? {
                if !values.insert(value) {
                    return Err(Error::custom("invalid entry: found duplicate value"));
                };
            }

            Ok(values)
        }
    }

    let visitor = SeqVisitor {
        marker: PhantomData,
        marker2: PhantomData,
    };
    deserializer.deserialize_seq(visitor)
}

/// Deserialize a map and return an error on duplicate keys
pub fn deserialize_map<'de, D, T, K, V>(deserializer: D) -> Result<T, D::Error>
where
    T: PreventDuplicateInsertsMap<K, V>,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct MapVisitor<T, K, V> {
        marker: PhantomData<T>,
        marker2: PhantomData<K>,
        marker3: PhantomData<V>,
    };

    impl<'de, T, K, V> Visitor<'de> for MapVisitor<T, K, V>
    where
        T: PreventDuplicateInsertsMap<K, V>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        #[inline]
        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut values = Self::Value::new(access.size_hint());

            while let Some((key, value)) = access.next_entry()? {
                if !values.insert(key, value) {
                    return Err(Error::custom("invalid entry: found duplicate key"));
                };
            }

            Ok(values)
        }
    }

    let visitor = MapVisitor {
        marker: PhantomData,
        marker2: PhantomData,
        marker3: PhantomData,
    };
    deserializer.deserialize_map(visitor)
}

impl<T> PreventDuplicateInsertsSet<T> for HashSet<T>
where
    T: Eq + Hash,
{
    #[inline]
    fn new(size_hint: Option<usize>) -> Self {
        match size_hint {
            Some(size) => Self::with_capacity(size),
            None => Self::new(),
        }
    }

    #[inline]
    fn insert(&mut self, value: T) -> bool {
        self.insert(value)
    }
}

impl<T> PreventDuplicateInsertsSet<T> for BTreeSet<T>
where
    T: Ord,
{
    #[inline]
    fn new(_size_hint: Option<usize>) -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, value: T) -> bool {
        self.insert(value)
    }
}

impl<K, V> PreventDuplicateInsertsMap<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    #[inline]
    fn new(size_hint: Option<usize>) -> Self {
        match size_hint {
            Some(size) => Self::with_capacity(size),
            None => Self::new(),
        }
    }

    #[inline]
    fn insert(&mut self, key: K, value: V) -> bool {
        self.insert(key, value).is_none()
    }
}

impl<K, V> PreventDuplicateInsertsMap<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    #[inline]
    fn new(_size_hint: Option<usize>) -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, key: K, value: V) -> bool {
        self.insert(key, value).is_none()
    }
}
