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
#![doc(html_root_url = "https://docs.rs/serde_with/1.5.0-alpha.1")]

//! [![docs.rs badge](https://docs.rs/serde_with/badge.svg)](https://docs.rs/serde_with/)
//! [![crates.io badge](https://img.shields.io/crates/v/serde_with.svg)](https://crates.io/crates/serde_with/)
//! [![Build Status](https://github.com/jonasbb/serde_with/workflows/Rust%20CI/badge.svg)](https://github.com/jonasbb/serde_with)
//! [![codecov](https://codecov.io/gh/jonasbb/serde_with/branch/master/graph/badge.svg)](https://codecov.io/gh/jonasbb/serde_with)
//!
//! ---
//!
//! This crate provides custom de/serialization helpers to use in combination with [serde's with-annotation][with-annotation] and with the improved [`serde_as`][]-annotation.
//! Some common use cases are:
//!
//! * De/Serializing a type using the `Display` and `FromStr` traits, e.g., for `u8`, `url::Url`, or `mime::Mime`.
//!      Check [`DisplayFromStr`][] or [`serde_with::rust::display_fromstr`][display_fromstr] for details.
//! * Skip serializing all empty `Option` types with [`#[skip_serializing_none]`][skip_serializing_none].
//! * Apply a prefix to each fieldname of a struct, without changing the de/serialize implementations of the struct using [`with_prefix!`][].
//! * Deserialize a comma separated list like `#hash,#tags,#are,#great` into a `Vec<String>`.
//!      Check the documentation for [`serde_with::rust::StringWithSeparator::<CommaSeparator>`][StringWithSeparator].
//!
//! Check out the [**user guide**][user guide] to find out more tips and tricks about this crate.
//!
//! # Use `serde_with` in your Project
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.serde_with]
//! version = "1.5.0-alpha.1"
//! features = [ "..." ]
//! ```
//!
//! The crate contains different features for integration with other common crates.
//! Check the [feature flags][] section for information about all available features.
//!
//! # Examples
//!
//! Annotate your struct or enum to enable the custom de/serializer.
//!
//! ## `DisplayFromStr`
//!
//! ```rust
//! # #[cfg(feature = "macros")]
//! # use serde_derive::{Deserialize, Serialize};
//! # #[cfg(feature = "macros")]
//! # use serde_with::{serde_as, DisplayFromStr};
//! # #[cfg(feature = "macros")]
//! #[serde_as]
//! # #[derive(Debug, Eq, PartialEq)]
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     // Serialize with Display, deserialize with FromStr
//!     #[serde_as(as = "DisplayFromStr")]
//!     bar: u8,
//! }
//!
//! # #[cfg(all(feature = "macros", feature = "json"))] {
//! // This will serialize
//! # let foo =
//! Foo {bar: 12}
//! # ;
//!
//! // into this JSON
//! # let json = r#"
//! {"bar": "12"}
//! # "#;
//! # assert_eq!(json.replace(" ", "").replace("\n", ""), serde_json::to_string(&foo).unwrap());
//! # assert_eq!(foo, serde_json::from_str(&json).unwrap());
//! # }
//! ```
//!
//! ## `skip_serializing_none`
//!
//! This situation often occurs with JSON, but other formats also support optional fields.
//! If many fields are optional, putting the annotations on the structs can become tedious.
//!
//! ```rust
//! # #[cfg(feature = "macros")]
//! # use serde_derive::{Deserialize, Serialize};
//! # #[cfg(feature = "macros")]
//! # use serde_with::skip_serializing_none;
//! # #[cfg(feature = "macros")]
//! #[skip_serializing_none]
//! # #[derive(Debug, Eq, PartialEq)]
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     a: Option<usize>,
//!     b: Option<usize>,
//!     c: Option<usize>,
//!     d: Option<usize>,
//!     e: Option<usize>,
//!     f: Option<usize>,
//!     g: Option<usize>,
//! }
//!
//! # #[cfg(all(feature = "macros", feature = "json"))] {
//! // This will serialize
//! # let foo =
//! Foo {a: None, b: None, c: None, d: Some(4), e: None, f: None, g: Some(7)}
//! # ;
//!
//! // into this JSON
//! # let json = r#"
//! {"d": 4, "g": 7}
//! # "#;
//! # assert_eq!(json.replace(" ", "").replace("\n", ""), serde_json::to_string(&foo).unwrap());
//! # assert_eq!(foo, serde_json::from_str(&json).unwrap());
//! # }
//! ```
//!
//! ## Advanced `serde_as` usage
//!
//! This example is mainly supposed to highlight the flexibility of the `serde_as`-annotation compared to [serde's with-annotation][with-annotation].
//! More details about `serde_as` can be found in the [user guide][].
//!
//!
//! ```rust
//! # #[cfg(all(feature = "macros", feature = "hex"))]
//! # use {
//! #     serde_derive::{Deserialize, Serialize},
//! #     serde_with::{serde_as, DisplayFromStr, DurationSeconds, hex::Hex},
//! #     std::time::Duration,
//! #     std::collections::BTreeMap,
//! # };
//! # #[cfg(all(feature = "macros", feature = "hex"))]
//! #[serde_as]
//! # #[derive(Debug, Eq, PartialEq)]
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!      // Serialize them into a list of number as seconds
//!      #[serde_as(as = "Vec<DurationSeconds>")]
//!      durations: Vec<Duration>,
//!      // We can treat a Vec like a map with duplicates.
//!      // JSON only allows string keys, so convert i32 to strings
//!      // The bytes will be hex encoded
//!      #[serde_as(as = "BTreeMap<DisplayFromStr, Hex>")]
//!      bytes: Vec<(i32, Vec<u8>)>,
//! }
//!
//! # #[cfg(all(feature = "macros", feature = "json", feature = "hex"))] {
//! // This will serialize
//! # let foo =
//! Foo {
//!     durations: vec![Duration::new(5, 0), Duration::new(3600, 0), Duration::new(0, 0)],
//!     bytes: vec![
//!         (1, vec![0, 1, 2]),
//!         (-100, vec![100, 200, 255]),
//!         (1, vec![0, 111, 222]),
//!     ],
//! }
//! # ;
//!
//! // into this JSON
//! # let json = r#"
//! {
//!     "durations": [5, 3600, 0],
//!     "bytes": {
//!         "1": "000102",
//!         "-100": "64c8ff",
//!         "1": "006fde"
//!     }
//! }
//! # "#;
//! # assert_eq!(json.replace(" ", "").replace("\n", ""), serde_json::to_string(&foo).unwrap());
//! # assert_eq!(foo, serde_json::from_str(&json).unwrap());
//! # }
//! ```
//!
//! [`DisplayFromStr`]: https://docs.rs/serde_with/*/serde_with/struct.DisplayFromStr.html
//! [`serde_as`]: https://docs.rs/serde_with/*/serde_with/guide/index.html
//! [`with_prefix!`]: https://docs.rs/serde_with/*/serde_with/macro.with_prefix.html
//! [display_fromstr]: https://docs.rs/serde_with/*/serde_with/rust/display_fromstr/index.html
//! [feature flags]: https://docs.rs/serde_with/*/serde_with/guide/feature_flags/index.html
//! [skip_serializing_none]: https://docs.rs/serde_with/*/serde_with/attr.skip_serializing_none.html
//! [StringWithSeparator]: https://docs.rs/serde_with/*/serde_with/rust/struct.StringWithSeparator.html
//! [user guide]: https://docs.rs/serde_with/*/serde_with/guide/index.html
//! [with-annotation]: https://serde.rs/field-attrs.html#with

#[doc(hidden)]
pub extern crate serde;

#[cfg(feature = "chrono")]
pub mod chrono;
pub mod de;
mod duplicate_key_impls;
mod flatten_maybe;
pub mod formats;
#[cfg(feature = "hex")]
pub mod hex;
#[cfg(feature = "json")]
pub mod json;
pub mod rust;
pub mod ser;
mod utils;
#[doc(hidden)]
pub mod with_prefix;

// Taken from shepmaster/snafu
// Originally licensed as MIT+Apache 2
// https://github.com/shepmaster/snafu/blob/fd37d79d4531ed1d3eebffad0d658928eb860cfe/src/lib.rs#L121-L165
#[cfg(feature = "guide")]
macro_rules! generate_guide {
    (pub mod $name:ident; $($rest:tt)*) => {
        generate_guide!(@gen ".", pub mod $name { } $($rest)*);
    };
    (pub mod $name:ident { $($children:tt)* } $($rest:tt)*) => {
        generate_guide!(@gen ".", pub mod $name { $($children)* } $($rest)*);
    };
    (@gen $prefix:expr, ) => {};
    (@gen $prefix:expr, pub mod $name:ident; $($rest:tt)*) => {
        generate_guide!(@gen $prefix, pub mod $name { } $($rest)*);
    };
    (@gen $prefix:expr, @code pub mod $name:ident; $($rest:tt)*) => {
        pub mod $name;
        generate_guide!(@gen $prefix, $($rest)*);
    };
    (@gen $prefix:expr, pub mod $name:ident { $($children:tt)* } $($rest:tt)*) => {
        doc_comment::doc_comment! {
            include_str!(concat!($prefix, "/", stringify!($name), ".md")),
            pub mod $name {
                generate_guide!(@gen concat!($prefix, "/", stringify!($name)), $($children)*);
            }
        }
        generate_guide!(@gen $prefix, $($rest)*);
    };
}

#[cfg(feature = "guide")]
generate_guide! {
    pub mod guide {
        pub mod migrating;
        pub mod feature_flags;
    }
}

#[doc(inline)]
pub use crate::{de::DeserializeAs, ser::SerializeAs};
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

/// Adapter to convert from `serde_as` to the serde traits.
///
/// The `As` type adapter allows to use types implementing [`DeserializeAs`][] or [`SerializeAs`][] in place of serde's with-annotation.
/// The with-annotation allows to run custom code when de/serializing, however it is quite inflexible.
/// The traits [`DeserializeAs`][]/[`SerializeAs`][] are more flexible, as they allow composition and nesting of types to create more complex de/serialization behavior.
/// However, they are not directly compatible with serde, as they are not provided by serde.
/// The `As` type adapter makes them compatible, by forwarding the function calls to `serialize`/`deserialize` to the corresponding functions `serialize_as` and `deserialize_as`.
///
/// It is not required to use this type directly.
/// Instead, it is highly encouraged to use the [`#[serde_as]`][serde_as] attribute since it includes further usability improvements.
/// If the use of the use of the proc-macro is not acceptable, then `As` can be used directly with serde.
///
/// ```rust
/// # use serde_derive::{Deserialize, Serialize};
/// # use serde_with::{As, DisplayFromStr};
/// #
/// #[derive(Deserialize, Serialize)]
/// # struct S {
/// // Serialize numbers as sequence of strings, using Display and FromStr
/// #[serde(with = "As::<Vec<DisplayFromStr>>")]
/// field: Vec<u8>,
/// # }
/// ```
/// If the normal `Deserialize`/`Serialize` traits should be used, the placeholder type [`Same`] can be used.
/// It implements [`DeserializeAs`][]/[`SerializeAs`][], when the underlying type implements `Deserialize`/`Serialize`.
///
/// ```rust
/// # use serde_derive::{Deserialize, Serialize};
/// # use serde_with::{As, DisplayFromStr, Same};
/// # use std::collections::BTreeMap;
/// #
/// #[derive(Deserialize, Serialize)]
/// # struct S {
/// // Serialize map, turn keys into strings but keep type of value
/// #[serde(with = "As::<BTreeMap<DisplayFromStr, Same>>")]
/// field: BTreeMap<u8, i32>,
/// # }
/// ```
///
/// [serde_as]: https://docs.rs/serde_with/*/serde_with/attr.serde_as.html
#[derive(Copy, Clone, Debug, Default)]
pub struct As<T>(PhantomData<T>);

impl<T> As<T> {
    /// Serialize type `T` using [`SerializeAs`][]
    ///
    /// The function signature is compatible with [serde's with-annotation][with-annotation].
    ///
    /// [with-annotation]: https://serde.rs/field-attrs.html#with
    pub fn serialize<S, I>(value: &I, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: SerializeAs<I>,
    {
        T::serialize_as(value, serializer)
    }

    /// Deserialize type `T` using [`DeserializeAs`][]
    ///
    /// The function signature is compatible with [serde's with-annotation][with-annotation].
    ///
    /// [with-annotation]: https://serde.rs/field-attrs.html#with
    pub fn deserialize<'de, D, I>(deserializer: D) -> Result<I, D::Error>
    where
        T: DeserializeAs<'de, I>,
        D: Deserializer<'de>,
    {
        T::deserialize_as(deserializer)
    }
}

/// Adapter to convert from `serde_as` to the serde traits.
///
/// This is the counter-type to [`As`][].
/// It can be used whenever a type implementing [`DeserializeAs`][]/[`SerializeAs`][] is required but the normal `Deserialize`/`Serialize` traits should be used.
/// Check [`As`] for an example.
#[derive(Copy, Clone, Debug, Default)]
pub struct Same;

/// De/Serialize using [`Display`] and [`FromStr`] implementation
///
/// This allows to deserialize a string as a number.
/// It can be very useful for serialization formats like JSON, which do not support integer
/// numbers and have to resort to strings to represent them.
///
/// Another use case is types with [`Display`] and [`FromStr`] implementations, but without serde
/// support, which can be found in some crates.
///
/// The same functionality is also available as [`serde_with::rust::display_fromstr`][crate::rust::display_fromstr] compatible with serde's with-annotation.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "macros")] {
/// # use serde_derive::{Deserialize, Serialize};
/// # use serde_json::json;
/// # use serde_with::{serde_as, DisplayFromStr};
/// #
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde_as(as = "DisplayFromStr")]
///     mime: mime::Mime,
///     #[serde_as(as = "DisplayFromStr")]
///     number: u32,
/// }
///
/// let v: A = serde_json::from_value(json!({
///     "mime": "text/plain",
///     "number": "159",
/// })).unwrap();
/// assert_eq!(mime::TEXT_PLAIN, v.mime);
/// assert_eq!(159, v.number);
///
/// let x = A {
///     mime: mime::STAR_STAR,
///     number: 777,
/// };
/// assert_eq!(json!({ "mime": "*/*", "number": "777" }), serde_json::to_value(&x).unwrap());
/// # }
/// ```
///
/// [`Display`]: std::fmt::Display
/// [`FromStr`]: std::str::FromStr
#[derive(Copy, Clone, Debug, Default)]
pub struct DisplayFromStr;

/// De/Serialize a [`Option`]`<`[`String`]`>` type while transforming the empty string to [`None`]
///
/// Convert an [`Option`]`<T>` from/to string using [`FromStr`] and [`AsRef`]`<`[`str`]`>` implementations.
/// An empty string is deserialized as [`None`] and a [`None`] vice versa.
///
/// The same functionality is also available as [`serde_with::rust::string_empty_as_none`][crate::rust::string_empty_as_none] compatible with serde's with-annotation.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "macros")] {
/// # use serde_derive::{Deserialize, Serialize};
/// # use serde_json::json;
/// # use serde_with::{serde_as, NoneAsEmptyString};
/// #
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde_as(as = "NoneAsEmptyString")]
///     tags: Option<String>,
/// }
///
/// let v: A = serde_json::from_value(json!({ "tags": "" })).unwrap();
/// assert_eq!(None, v.tags);
///
/// let v: A = serde_json::from_value(json!({ "tags": "Hi" })).unwrap();
/// assert_eq!(Some("Hi".to_string()), v.tags);
///
/// let x = A {
///     tags: Some("This is text".to_string()),
/// };
/// assert_eq!(json!({ "tags": "This is text" }), serde_json::to_value(&x).unwrap());
///
/// let x = A {
///     tags: None,
/// };
/// assert_eq!(json!({ "tags": "" }), serde_json::to_value(&x).unwrap());
/// # }
/// ```
///
/// [`FromStr`]: std::str::FromStr
#[derive(Copy, Clone, Debug, Default)]
pub struct NoneAsEmptyString;

#[derive(Copy, Clone, Debug, Default)]
pub struct DefaultOnError<T>(PhantomData<T>);

#[derive(Copy, Clone, Debug, Default)]
pub struct BytesOrString;

#[derive(Copy, Clone, Debug, Default)]
pub struct DurationSeconds<
    FORMAT: formats::Format = u64,
    STRICTNESS: formats::Strictness = formats::Strict,
>(PhantomData<(FORMAT, STRICTNESS)>);

#[derive(Copy, Clone, Debug, Default)]
pub struct DurationSecondsWithFrac<
    FORMAT: formats::Format = f64,
    STRICTNESS: formats::Strictness = formats::Strict,
>(PhantomData<(FORMAT, STRICTNESS)>);
