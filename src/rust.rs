//! De/Serialization for Rust's builtin and std types

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
///
/// [Display]: ::std::fmt::Display
/// [FromStr]: ::std::str::FromStr
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
        <T as FromStr>::Err: Display,
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
