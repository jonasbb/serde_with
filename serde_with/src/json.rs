//! De/Serialization of JSON
//!
//! This modules is only available when using the `json` feature of the crate.

use crate::{de::DeserializeAs, ser::SerializeAs};
use core::{fmt, marker::PhantomData};
use serde::{
    de,
    de::{DeserializeOwned, Deserializer, Visitor},
    ser,
    ser::{Serialize, Serializer},
};

/// Serialize value as string containing JSON
///
/// *Note*: This type is not necessary for normal usage of serde with JSON.
/// It is only required if the serialized format contains a string, which itself contains JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "macros")] {
/// # use serde::{Deserialize, Serialize};
/// # use serde_with::{serde_as, json::JsonString};
/// #
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct A {
///     #[serde_as(as = "JsonString")]
///     other_struct: B,
/// }
/// #[derive(Deserialize, Serialize)]
/// struct B {
///     value: usize,
/// }
///
/// let v: A = serde_json::from_str(r#"{"other_struct":"{\"value\":5}"}"#).unwrap();
/// assert_eq!(5, v.other_struct.value);
///
/// let x = A {
///     other_struct: B { value: 10 },
/// };
/// assert_eq!(
///     r#"{"other_struct":"{\"value\":10}"}"#,
///     serde_json::to_string(&x).unwrap()
/// );
/// # }
/// ```
pub struct JsonString;

impl<T> SerializeAs<T> for JsonString
where
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serde_json::to_string(source).map_err(ser::Error::custom)?;
        serializer.serialize_str(&*s)
    }
}

impl<'de, T> DeserializeAs<'de, T> for JsonString
where
    T: DeserializeOwned,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper<S: DeserializeOwned>(PhantomData<S>);

        impl<'de, S> Visitor<'de> for Helper<S>
        where
            S: DeserializeOwned,
        {
            type Value = S;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("valid json object")
            }

            fn visit_str<E>(self, value: &str) -> Result<S, E>
            where
                E: de::Error,
            {
                serde_json::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(Helper(PhantomData))
    }
}
