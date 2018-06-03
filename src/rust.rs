//! De/Serialization for Rust's builtin and std types

use serde::{de, ser, Deserialize};
use std::fmt::Display;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;
use Separator;

/// De/Serialize using [Display] and [FromStr] implementation
///
/// # Examples
///
/// ```
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # use std::net::Ipv4Addr;
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde(with = "serde_with::rust::display_fromstr")]
///     address: Ipv4Addr,
///     #[serde(with = "serde_with::rust::display_fromstr")]
///     b: bool,
/// }
///
/// # fn main() {
/// let v: A = serde_json::from_str(r#"{
///     "address": "192.168.2.1",
///     "b": "true"
/// }"#).unwrap();
/// assert_eq!(Ipv4Addr::new(192, 168, 2, 1), v.address);
/// assert!(v.b);
///
/// let x = A {
///     address: Ipv4Addr::new(127, 53, 0, 1),
///     b: false,
/// };
/// assert_eq!(r#"{"address":"127.53.0.1","b":"false"}"#, serde_json::to_string(&x).unwrap());
/// # }
/// ```
pub mod display_fromstr {
    use serde::de::{Deserializer, Error, Visitor};
    use serde::ser::Serializer;
    use std::fmt;
    use std::fmt::Display;
    use std::marker::PhantomData;
    use std::str::FromStr;

    /// Deserialize T using [FromStr]
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: Display,
    {
        struct Helper<S>(PhantomData<S>);

        impl<'de, S> Visitor<'de> for Helper<S>
        where
            S: FromStr,
            <S as FromStr>::Err: Display,
        {
            type Value = S;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "valid json object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                value.parse::<Self::Value>().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(Helper(PhantomData))
    }

    /// Serialize T using [Display]
    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.serialize_str(&*value.to_string())
    }
}

/// De/Serialize a delimited collection using [Display] and [FromStr] implementation
///
/// You can define an arbitrary seperator, by specifying a type which implements [Separator].
/// Some common ones, like space and comma are already predefined and you can find them [here][Separator].
///
/// An empty string deserializes as an empty collection.
///
/// # Examples
///
/// ```
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// use serde_with::{CommaSeparator, SpaceSeparator};
/// use std::collections::BTreeSet;
///
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde(with = "serde_with::rust::StringWithSeparator::<SpaceSeparator>")]
///     tags: Vec<String>,
///     #[serde(with = "serde_with::rust::StringWithSeparator::<CommaSeparator>")]
///     more_tags: BTreeSet<String>,
/// }
///
/// # fn main() {
/// let v: A = serde_json::from_str(r##"{
///     "tags": "#hello #world",
///     "more_tags": "foo,bar,bar"
/// }"##).unwrap();
/// assert_eq!(vec!["#hello", "#world"], v.tags);
/// assert_eq!(2, v.more_tags.len());
///
/// let x = A {
///     tags: vec!["1".to_string(), "2".to_string(), "3".to_string()],
///     more_tags: BTreeSet::new(),
/// };
/// assert_eq!(r#"{"tags":"1 2 3","more_tags":""}"#, serde_json::to_string(&x).unwrap());
/// # }
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StringWithSeparator<Sep>(PhantomData<Sep>);

impl<Sep> StringWithSeparator<Sep>
where
    Sep: Separator,
{
    pub fn serialize<S, T, V>(values: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: IntoIterator<Item = V>,
        V: Display,
    {
        let mut s = String::new();
        for v in values {
            s.push_str(&*v.to_string());
            s.push_str(Sep::separator());
        }
        serializer.serialize_str(if !s.is_empty() {
            // remove trailing separator if present
            &s[..s.len() - Sep::separator().len()]
        } else {
            &s[..]
        })
    }

    pub fn deserialize<'de, D, T, V>(deserializer: D) -> Result<T, D::Error>
    where
        D: de::Deserializer<'de>,
        T: FromIterator<V>,
        V: FromStr,
        V::Err: Display,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None.into_iter().collect())
        } else {
            s.split(Sep::separator())
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()
                .map_err(de::Error::custom)
        }
    }
}

/// Makes a distinction between a missing, unset, or existing value
///
/// Some serialization formats make a distinction between missing fields, fields with a `null`
/// value, and existing values. One such format is JSON. By default it is not easily possible to
/// differentiate between a missing value and a field which is `null`, as they deserialize to the
/// same value. This helper changes it, by using an `Option<Option<T>>` to deserialize into.
///
/// * `None`: Represents a missing value.
/// * `Some(None)`: Represents a `null` value.
/// * `Some(Some(value))`: Represents an existing value.
///
/// # Examples
///
/// ```rust
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # #[derive(Debug, PartialEq, Eq)]
/// #[derive(Deserialize, Serialize)]
/// struct Doc {
///     #[serde(
///         default,                                    // <- important for deserialization
///         skip_serializing_if = "Option::is_none",    // <- important for serialization
///         with = "::serde_with::rust::double_option",
///     )]
///     a: Option<Option<u8>>,
/// }
/// # fn main() {
/// // Missing Value
/// let s = r#"{}"#;
/// assert_eq!(Doc {a: None}, serde_json::from_str(s).unwrap());
/// assert_eq!(s, serde_json::to_string(&Doc {a: None}).unwrap());
///
/// // Unset Value
/// let s = r#"{"a":null}"#;
/// assert_eq!(Doc {a: Some(None)}, serde_json::from_str(s).unwrap());
/// assert_eq!(s, serde_json::to_string(&Doc {a: Some(None)}).unwrap());
///
/// // Existing Value
/// let s = r#"{"a":5}"#;
/// assert_eq!(Doc {a: Some(Some(5))}, serde_json::from_str(s).unwrap());
/// assert_eq!(s, serde_json::to_string(&Doc {a: Some(Some(5))}).unwrap());
/// # }
/// ```
#[cfg_attr(feature = "cargo-clippy", allow(option_option))]
pub mod double_option {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Some)
    }

    pub fn serialize<S, T>(values: &Option<Option<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match values {
            None => serializer.serialize_unit(),
            Some(None) => serializer.serialize_none(),
            Some(Some(v)) => serializer.serialize_some(&v),
        }
    }
}
