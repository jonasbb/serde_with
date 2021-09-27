#![deny(
    missing_copy_implementations,
    // missing_crate_level_docs, not available in MSRV
    // missing_debug_implementations,
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![warn(rust_2018_idioms)]
#![doc(test(attr(forbid(unsafe_code))))]
#![doc(test(attr(deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
))))]
#![doc(test(attr(warn(rust_2018_idioms))))]
// Not needed for 2018 edition and conflicts with `rust_2018_idioms`
#![doc(test(no_crate_inject))]
#![doc(html_root_url = "https://docs.rs/serde_content/1.0.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! TODO

extern crate serde;

pub mod de;
pub mod ser;

mod size_hint {
    pub(crate) fn from_bounds<I>(iter: &I) -> Option<usize>
    where
        I: Iterator,
    {
        helper(iter.size_hint())
    }

    #[inline]
    pub(crate) fn cautious(hint: Option<usize>) -> usize {
        std::cmp::min(hint.unwrap_or(0), 4096)
    }

    fn helper(bounds: (usize, Option<usize>)) -> Option<usize> {
        match bounds {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
