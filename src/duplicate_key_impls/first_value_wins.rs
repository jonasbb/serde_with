use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
};

pub trait DuplicateInsertsFirstWinsSet<T> {
    fn new(size_hint: Option<usize>) -> Self;

    /// Insert the value into the set, if there is not already an existing value
    fn insert(&mut self, value: T);
}

pub trait DuplicateInsertsFirstWinsMap<K, V> {
    fn new(size_hint: Option<usize>) -> Self;

    /// Insert the value into the set, if there is not already an existing value
    fn insert(&mut self, key: K, value: V);
}

impl<T> DuplicateInsertsFirstWinsSet<T> for HashSet<T>
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
    fn insert(&mut self, value: T) {
        // Hashset already fullfils the contract and always keeps the first value
        self.insert(value);
    }
}

impl<T> DuplicateInsertsFirstWinsSet<T> for BTreeSet<T>
where
    T: Ord,
{
    #[inline]
    fn new(_size_hint: Option<usize>) -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, value: T) {
        // BTreeSet already fullfils the contract and always keeps the first value
        self.insert(value);
    }
}

impl<K, V> DuplicateInsertsFirstWinsMap<K, V> for HashMap<K, V>
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
    fn insert(&mut self, key: K, value: V) {
        use std::collections::hash_map::Entry;

        match self.entry(key) {
            // we want to keep the first value, so do nothing
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }
}

impl<K, V> DuplicateInsertsFirstWinsMap<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    #[inline]
    fn new(_size_hint: Option<usize>) -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, key: K, value: V) {
        use std::collections::btree_map::Entry;

        match self.entry(key) {
            // we want to keep the first value, so do nothing
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }
}
