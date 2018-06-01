#![deny(
    missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications,
    variant_size_differences
)]
#![allow(missing_docs)]

#[cfg(feature = "chrono")]
extern crate chrono as chrono_crate;
extern crate serde;
#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "json")]
pub mod json;
pub mod rust;
pub mod option;

/// Seperator for string-based collection de/serialization
pub trait Separator {
    /// Return the string delimiting two elements in the string-based collection
    fn separator() -> &'static str;
}

/// Predefined seperator using a single space
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SpaceSeparator;

impl Separator for SpaceSeparator {
    #[inline]
    fn separator() -> &'static str {
        " "
    }
}

/// Predefined seperator using a single comma
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CommaSeparator;

impl Separator for CommaSeparator {
    #[inline]
    fn separator() -> &'static str {
        ","
    }
}

