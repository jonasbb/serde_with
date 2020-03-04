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
