pub(crate) mod duration;

use crate::prelude::*;
use num_traits::ToPrimitive as _;

/// Re-Implementation of `serde::private::de::size_hint::cautious`
#[cfg(feature = "alloc")]
#[inline]
pub(crate) fn size_hint_cautious<Element>(hint: Option<usize>) -> usize {
    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

    if core::mem::size_of::<Element>() == 0 {
        0
    } else {
        core::cmp::min(
            hint.unwrap_or(0),
            MAX_PREALLOC_BYTES / core::mem::size_of::<Element>(),
        )
    }
}

/// Re-Implementation of `serde::private::de::size_hint::from_bounds`
#[cfg(feature = "alloc")]
#[inline]
pub fn size_hint_from_bounds<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    fn _size_hint_from_bounds(bounds: (usize, Option<usize>)) -> Option<usize> {
        match bounds {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
    _size_hint_from_bounds(iter.size_hint())
}

pub(crate) const NANOS_PER_SEC: u128 = 1_000_000_000;
pub(crate) const NANOS_PER_SEC_F64: f64 = 1_000_000_000.0;
// pub(crate) const NANOS_PER_MILLI: u32 = 1_000_000;
// pub(crate) const NANOS_PER_MICRO: u32 = 1_000;
// pub(crate) const MILLIS_PER_SEC: u64 = 1_000;
// pub(crate) const MICROS_PER_SEC: u64 = 1_000_000;
#[expect(clippy::as_conversions)]
pub(crate) const U64_MAX: u128 = u64::MAX as u128;
#[expect(clippy::as_conversions)]
pub(crate) const U64_MAX_FLOAT: f64 = u64::MAX as f64;

pub(crate) struct MapIter<'de, A, K, V> {
    pub(crate) access: A,
    marker: PhantomData<(&'de (), K, V)>,
}

impl<'de, A, K, V> MapIter<'de, A, K, V> {
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

impl<'de, A, K, V> Iterator for MapIter<'de, A, K, V>
where
    A: MapAccess<'de>,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Item = Result<(K, V), A::Error>;

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

pub(crate) struct SeqIter<'de, A, T> {
    access: A,
    marker: PhantomData<(&'de (), T)>,
}

impl<'de, A, T> SeqIter<'de, A, T> {
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

impl<'de, A, T> Iterator for SeqIter<'de, A, T>
where
    A: SeqAccess<'de>,
    T: Deserialize<'de>,
{
    type Item = Result<T, A::Error>;

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

pub(crate) fn duration_signed_from_secs_f64(mut secs: f64) -> Result<DurationSigned, &'static str> {
    if !secs.is_finite() {
        return Err("got non-finite value when converting float to duration");
    }
    if secs.trunc() > U64_MAX_FLOAT {
        return Err("overflow when converting float to duration");
    }
    let mut sign = Sign::Positive;
    if secs < 0.0 {
        secs = -secs;
        sign = Sign::Negative;
    }
    Ok(DurationSigned::new(
        sign,
        secs.trunc() as u64,
        (secs.fract() * NANOS_PER_SEC_F64) as u32,
    ))
}

/// Collect an array of a fixed size from an iterator.
///
/// # Safety
/// The code follow exactly the pattern of initializing an array element-by-element from the standard library.
/// <https://doc.rust-lang.org/nightly/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element>
pub(crate) fn array_from_iterator<I, T, E, const N: usize>(
    mut iter: I,
    expected: &dyn Expected,
) -> Result<[T; N], E>
where
    I: Iterator<Item = Result<T, E>>,
    E: DeError,
{
    use core::mem::MaybeUninit;

    fn drop_array_elems<T, const N: usize>(num: usize, mut arr: [MaybeUninit<T>; N]) {
        arr[..num].iter_mut().for_each(|elem| {
            // TODO This would be better with assume_init_drop nightly function
            // https://github.com/rust-lang/rust/issues/63567
            unsafe { core::ptr::drop_in_place(elem.as_mut_ptr()) };
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
    for (idx, elem) in arr[..].iter_mut().enumerate() {
        *elem = match iter.next() {
            Some(Ok(value)) => MaybeUninit::new(value),
            Some(Err(err)) => {
                drop_array_elems(idx, arr);
                return Err(err);
            }
            None => {
                drop_array_elems(idx, arr);
                return Err(DeError::invalid_length(idx, expected));
            }
        };
    }

    // Everything is initialized. Transmute the array to the
    // initialized type.
    // A normal transmute is not possible because of:
    // https://github.com/rust-lang/rust/issues/61956
    Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&arr) })
}

/// Writer that writes into a `&mut [u8]` while checking the length of the buffer
struct BufWriter<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}

impl<'a> BufWriter<'a> {
    fn new(bytes: &'a mut [u8]) -> Self {
        BufWriter { bytes, offset: 0 }
    }

    fn into_str(self) -> &'a str {
        let slice = &self.bytes[..self.offset];
        core::str::from_utf8(slice)
            .unwrap_or("Failed to extract valid string from BufWriter. This should never happen.")
    }
}

impl core::fmt::Write for BufWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > self.bytes.len() - self.offset {
            Err(fmt::Error)
        } else {
            self.bytes[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
            self.offset += s.len();
            Ok(())
        }
    }
}

// 58 chars is long enough for any i128 and u128 value
pub(crate) fn get_unexpected_i128(value: i128, buf: &mut [u8; 58]) -> Unexpected<'_> {
    let mut writer = BufWriter::new(buf);
    fmt::Write::write_fmt(&mut writer, format_args!("integer `{value}` as i128")).unwrap();
    Unexpected::Other(writer.into_str())
}

// 58 chars is long enough for any i128 and u128 value
pub(crate) fn get_unexpected_u128(value: u128, buf: &mut [u8; 58]) -> Unexpected<'_> {
    let mut writer = BufWriter::new(buf);
    fmt::Write::write_fmt(&mut writer, format_args!("integer `{value}` as u128")).unwrap();
    Unexpected::Other(writer.into_str())
}
