use super::*;
use crate::utils::{MapIter, SeqIter};
use serde::de::*;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryInto;
use std::fmt;
use std::mem::MaybeUninit;

// TODO this should probably be moved into the utils module when const generics are available for MSRV

/// # Safety
/// The code follow exactly the pattern of initializing an array element-by-element from the standard library.
/// <https://doc.rust-lang.org/nightly/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element>
fn array_from_iterator<I, T, E, const N: usize>(
    mut iter: I,
    expected: &dyn Expected,
) -> Result<[T; N], E>
where
    I: Iterator<Item = Result<T, E>>,
    E: Error,
{
    fn drop_array_elems<T, const N: usize>(num: usize, mut arr: [MaybeUninit<T>; N]) {
        (&mut arr[0..num]).iter_mut().for_each(|elem| {
            // TODO This would be better with assume_init_drop nightly function
            // https://github.com/rust-lang/rust/issues/63567
            unsafe { std::ptr::drop_in_place(elem.as_mut_ptr()) };
        });
    }

    // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
    // safe because the type we are claiming to have initialized here is a
    // bunch of `MaybeUninit`s, which do not require initialization.
    //
    // TODO could be simplified with nightly maybe_uninit_uninit_array feature
    // https://doc.rust-lang.org/nightly/std/mem/union.MaybeUninit.html#method.uninit_array
    let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
    // assignment instead of `ptr::write` does not cause the old
    // uninitialized value to be dropped. Also if there is a panic during
    // this loop, we have a memory leak, but there is no memory safety
    // issue.
    for (idx, elem) in (&mut arr[..]).iter_mut().enumerate() {
        *elem = match iter.next() {
            Some(Ok(value)) => MaybeUninit::new(value),
            Some(Err(err)) => {
                drop_array_elems(idx, arr);
                return Err(err);
            }
            None => {
                drop_array_elems(idx, arr);
                return Err(Error::invalid_length(idx, expected));
            }
        };
    }

    // Everything is initialized. Transmute the array to the
    // initialized type.
    // A normal transmute is not possible because of:
    // https://github.com/rust-lang/rust/issues/61956
    Ok(unsafe { std::mem::transmute_copy::<_, [T; N]>(&arr) })
}

impl<'de, T, As, const N: usize> DeserializeAs<'de, [T; N]> for [As; N]
where
    As: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor<T, const M: usize>(PhantomData<T>);

        impl<'de, T, As, const M: usize> Visitor<'de> for ArrayVisitor<DeserializeAsWrap<T, As>, M>
        where
            As: DeserializeAs<'de, T>,
        {
            type Value = [T; M];

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_fmt(format_args!("an array of size {}", M))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                array_from_iterator(
                    SeqIter::new(seq).map(|res: Result<DeserializeAsWrap<T, As>, A::Error>| {
                        res.map(|t| t.into_inner())
                    }),
                    &self,
                )
            }
        }

        deserializer.deserialize_tuple(N, ArrayVisitor::<DeserializeAsWrap<T, As>, N>(PhantomData))
    }
}

macro_rules! tuple_seq_as_map_impl_intern {
    ($tyorig:ty, $ty:ident <KAs, VAs>) => {
        #[allow(clippy::implicit_hasher)]
        impl<'de, K, KAs, V, VAs, const N: usize> DeserializeAs<'de, $tyorig> for $ty<KAs, VAs>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$tyorig, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MapVisitor<K, KAs, V, VAs, const M: usize> {
                    marker: PhantomData<(K, KAs, V, VAs)>,
                }

                impl<'de, K, KAs, V, VAs, const M: usize> Visitor<'de> for MapVisitor<K, KAs, V, VAs, M>
                where
                    KAs: DeserializeAs<'de, K>,
                    VAs: DeserializeAs<'de, V>,
                {
                    type Value = [(K, V); M];

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_fmt(format_args!("a map of length {}", M))
                    }

                    fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        array_from_iterator(MapIter::new(access).map(
                            |res: Result<(DeserializeAsWrap<K, KAs>, DeserializeAsWrap<V, VAs>), A::Error>| {
                                res.map(|(k, v)| (k.into_inner(), v.into_inner()))
                            }
                        ), &self)
                    }
                }

                let visitor = MapVisitor::<K, KAs, V, VAs, N> {
                    marker: PhantomData,
                };
                deserializer.deserialize_map(visitor)
            }
        }
    }
}
tuple_seq_as_map_impl_intern!([(K, V); N], BTreeMap<KAs, VAs>);
tuple_seq_as_map_impl_intern!([(K, V); N], HashMap<KAs, VAs>);

impl<'de, const N: usize> DeserializeAs<'de, [u8; N]> for Bytes {
    fn deserialize_as<D>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor<const M: usize>;

        impl<'de, const M: usize> Visitor<'de> for ArrayVisitor<M> {
            type Value = [u8; M];

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_fmt(format_args!("an byte array of size {}", M))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                array_from_iterator(SeqIter::new(seq), &self)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.try_into()
                    .map_err(|_| Error::invalid_length(v.len(), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.as_bytes()
                    .try_into()
                    .map_err(|_| Error::invalid_length(v.len(), &self))
            }
        }

        deserializer.deserialize_bytes(ArrayVisitor::<N>)
    }
}

impl<'de, const N: usize> DeserializeAs<'de, Box<[u8; N]>> for Bytes {
    fn deserialize_as<D>(deserializer: D) -> Result<Box<[u8; N]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Bytes as DeserializeAs<'de, [u8; N]>>::deserialize_as(deserializer).map(Box::new)
    }
}
