use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
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
