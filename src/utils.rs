use crate::de::{impls::DeserializeAsWrap, DeserializeAs};
use serde::de::{MapAccess, SeqAccess};
use std::marker::PhantomData;

/// Re-Implementation of `serde::private::de::size_hint::cautious`
#[inline]
pub(crate) fn size_hint_cautious(hint: Option<usize>) -> usize {
    std::cmp::min(hint.unwrap_or(0), 4096)
}

pub(crate) const NANOS_PER_SEC: u32 = 1_000_000_000;
// pub(crate) const NANOS_PER_MILLI: u32 = 1_000_000;
// pub(crate) const NANOS_PER_MICRO: u32 = 1_000;
// pub(crate) const MILLIS_PER_SEC: u64 = 1_000;
// pub(crate) const MICROS_PER_SEC: u64 = 1_000_000;

pub(crate) struct MapIter<'de, A, K, KAs, V, VAs> {
    pub(crate) access: A,
    marker: PhantomData<(&'de (), K, KAs, V, VAs)>,
}

impl<'de, A, K, KAs, V, VAs> MapIter<'de, A, K, KAs, V, VAs> {
    pub(crate) fn new(access: A) -> Self
    where
        A: MapAccess<'de>,
    {
        Self {
            access,
            marker: PhantomData,
        }
    }
}

impl<'de, A, K, KAs, V, VAs> Iterator for MapIter<'de, A, K, KAs, V, VAs>
where
    A: MapAccess<'de>,
    KAs: DeserializeAs<'de, K>,
    VAs: DeserializeAs<'de, V>,
{
    #[allow(clippy::type_complexity)]
    type Item = Result<(DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>), A::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.access.next_entry().transpose()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.access.size_hint() {
            Some(size) => (size, Some(size)),
            None => (0, None),
        }
    }
}

pub(crate) struct SeqIter<'de, A, K, KAs, V, VAs> {
    access: A,
    marker: PhantomData<(&'de (), K, KAs, V, VAs)>,
}

impl<'de, A, K, KAs, V, VAs> SeqIter<'de, A, K, KAs, V, VAs> {
    pub(crate) fn new(access: A) -> Self
    where
        A: SeqAccess<'de>,
    {
        Self {
            access,
            marker: PhantomData,
        }
    }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.access.size_hint() {
            Some(size) => (size, Some(size)),
            None => (0, None),
        }
    }
}
