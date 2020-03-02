/// Re-Implementation of `serde::private::de::size_hint::cautious`
#[inline]
pub(crate) fn size_hint_cautious(hint: Option<usize>) -> usize {
    std::cmp::min(hint.unwrap_or(0), 4096)
}
