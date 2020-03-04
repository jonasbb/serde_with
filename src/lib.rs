#![deny(
    // missing_copy_implementations,
    // missing_debug_implementations,
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
// #![warn(rust_2018_idioms)]
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
#![doc(html_root_url = "https://docs.rs/serde_with/1.4.0")]

//! [![docs.rs badge](https://docs.rs/serde_with/badge.svg)](https://docs.rs/serde_with/)
//! [![crates.io badge](https://img.shields.io/crates/v/serde_with.svg)](https://crates.io/crates/serde_with/)
//! [![Build Status](https://github.com/jonasbb/serde_with/workflows/Rust%20CI/badge.svg)](https://github.com/jonasbb/serde_with)
//! [![codecov](https://codecov.io/gh/jonasbb/serde_with/branch/master/graph/badge.svg)](https://codecov.io/gh/jonasbb/serde_with)
//!
//! ---
//!
//! This crate provides custom de/serialization helpers to use in combination with [serde's with-annotation][with-annotation].
//!
//! Serde tracks a wishlist of similar helpers at [serde#553].
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.serde_with]
//! version = "1.4.0"
//! features = [ "..." ]
//! ```
//!
//! The crate is divided into different modules.
//! They contain helpers for external crates and must be enabled with the correspondig feature.
//!
//! Annotate your struct or enum to enable the custom de/serializer.
//!
//! ```rust
//! # use serde_derive::{Deserialize, Serialize};
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "serde_with::rust::display_fromstr")]
//!     bar: u8,
//! }
//! # fn main() {}
//! ```
//!
//! Most helpers implement both deserialize and serialize.
//! If you do not want to derive both, you can simply derive only the necessary parts.
//! If you want to mix different helpers, you can write your annotations like
//!
//! ```rust
//! # use serde_derive::{Deserialize, Serialize};
//! # #[cfg(feature = "json")]
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(
//!         deserialize_with = "serde_with::rust::display_fromstr::deserialize",
//!         serialize_with = "serde_with::json::nested::serialize"
//!     )]
//!     bar: u8,
//! }
//! # fn main() {}
//! ```
//!
//! However, this will prohibit you from applying deserialize on the value returned by serializing a struct.
//!
//! # Attributes
//!
//! The crate comes with custom attributes, which futher extend how serde serialization can be customized.
//! They are enabled by default, but can be disabled, by removing the default features from this crate.
//!
//! [with-annotation]: https://serde.rs/field-attrs.html#with
//! [serde#553]: https://github.com/serde-rs/serde/issues/553

#[doc(hidden)]
pub extern crate serde;

#[cfg(feature = "chrono")]
pub mod chrono;
pub mod de;
mod duplicate_key_impls;
mod flatten_maybe;
#[cfg(feature = "hex")]
pub mod hex;
#[cfg(feature = "json")]
pub mod json;
pub mod rust;
pub mod ser;
mod utils;
#[doc(hidden)]
pub mod with_prefix;

use crate::{de::DeserializeAs, ser::SerializeAs};
use serde::{ser::Serialize, Deserializer, Serializer};
// Re-Export all proc_macros, as these should be seen as part of the serde_with crate
#[cfg(feature = "macros")]
#[doc(inline)]
pub use serde_with_macros::*;
use std::marker::PhantomData;

/// Separator for string-based collection de/serialization
pub trait Separator {
    /// Return the string delimiting two elements in the string-based collection
    fn separator() -> &'static str;
}

/// Predefined separator using a single space
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SpaceSeparator;

impl Separator for SpaceSeparator {
    #[inline]
    fn separator() -> &'static str {
        " "
    }
}

/// Predefined separator using a single comma
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CommaSeparator;

impl Separator for CommaSeparator {
    #[inline]
    fn separator() -> &'static str {
        ","
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct As<T>(PhantomData<T>);

impl<T> As<T> {
    pub fn serialize<S, I>(value: &I, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: SerializeAs<I>,
    {
        T::serialize_as(value, serializer)
    }

    pub fn deserialize<'de, D, I>(deserializer: D) -> Result<I, D::Error>
    where
        T: DeserializeAs<'de, I>,
        D: Deserializer<'de>,
    {
        T::deserialize_as(deserializer)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Same;

#[derive(Copy, Clone, Debug, Default)]
pub struct SameAs<T>(PhantomData<T>);

#[derive(Copy, Clone, Debug, Default)]
pub struct DisplayFromStr;

#[derive(Copy, Clone, Debug, Default)]
pub struct NoneAsEmptyString;

#[derive(Copy, Clone, Debug, Default)]
pub struct DefaultOnError<T>(PhantomData<T>);

#[derive(Copy, Clone, Debug, Default)]
pub struct BytesOrString;

pub trait Format {}
pub trait Strictness {}

#[derive(Copy, Clone, Debug, Default)]
pub struct Integer;
impl Format for Integer {}
impl Format for String {}

#[derive(Copy, Clone, Debug, Default)]
pub struct Strict;
impl Strictness for Strict {}

#[derive(Copy, Clone, Debug, Default)]
pub struct Flexible;
impl Strictness for Flexible {}

#[derive(Copy, Clone, Debug, Default)]
pub struct DurationSeconds<FORMAT: Format = Integer, STRICTNESS: Strictness = Strict>(
    PhantomData<(FORMAT, STRICTNESS)>,
);
