//! De/Serialization for Rust's builtin and std types

use serde::{de, ser, Deserialize};
use std::{fmt::Display, iter::FromIterator, marker::PhantomData, str::FromStr};
use Separator;

/// De/Serialize using [Display][] and [FromStr][] implementation
///
/// This allows to deserialize a string as a number.
/// It can be very useful for serialization formats like JSON, which do not support integer
/// numbers and have to resort to strings to represent them.
///
/// [Display]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
/// [FromStr]: https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
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
    use serde::{
        de::{Deserializer, Error, Visitor},
        ser::Serializer,
    };
    use std::{
        fmt::{self, Display},
        marker::PhantomData,
        str::FromStr,
    };

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

/// De/Serialize a delimited collection using [Display][] and [FromStr][] implementation
///
/// You can define an arbitrary separator, by specifying a type which implements [Separator][].
/// Some common ones, like space and comma are already predefined and you can find them [here][Separator].
///
/// An empty string deserializes as an empty collection.
///
/// [Display]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
/// [FromStr]: https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
/// [Separator]: ../trait.Separator.html
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
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

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

/// Serialize inner value if `Some(T)`. If `None`, serialize the unit struct `()`.
///
/// When used in conjunction with `skip_serializing_if = "Option::is_none"` and
/// `default`, you can build an optional value by skipping if it is `None`, or serializing its
/// inner value if `Some(T)`.
///
/// Not all serialization formats easily support optional values.
/// While JSON uses the `Option` type to represent optional values and only serializes the inner
/// part of the `Some()`, other serialization formats, such as [RON][], choose to serialize the
/// `Some` around a value.
/// This helper helps building a truly optional value for such serializers.
///
/// [RON]: https://github.com/ron-rs/ron
///
/// # Example
///
/// ```rust
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # extern crate ron;
/// # #[derive(Debug, Eq, PartialEq)]
/// #[derive(Deserialize, Serialize)]
/// struct Doc {
///     mandatory: usize,
///     #[serde(
///         default,                                    // <- important for deserialization
///         skip_serializing_if = "Option::is_none",    // <- important for serialization
///         with = "::serde_with::rust::unwrap_or_skip",
///     )]
///     optional: Option<usize>,
/// }
/// # fn main() {
///
/// // Transparently add/remove Some() wrapper
/// # let pretty_config = ron::ser::PrettyConfig::default();
/// let s = r#"(
///     mandatory: 1,
///     optional: 2,
/// )"#;
/// let v = Doc {
///     mandatory: 1,
///     optional: Some(2),
/// };
/// assert_eq!(v, ron::de::from_str(s).unwrap());
/// assert_eq!(s, ron::ser::to_string_pretty(&v, pretty_config).unwrap());
///
/// // Missing values are deserialized as `None`
/// // while `None` values are skipped during serialization.
/// # let pretty_config = ron::ser::PrettyConfig::default();
/// let s = r#"(
///     mandatory: 1,
/// )"#;
/// let v = Doc {
///     mandatory: 1,
///     optional: None,
/// };
/// assert_eq!(v, ron::de::from_str(s).unwrap());
/// assert_eq!(s, ron::ser::to_string_pretty(&v, pretty_config).unwrap());
/// # }
/// ```
pub mod unwrap_or_skip {
    use serde::{
        de::{DeserializeOwned, Deserializer},
        ser::{Serialize, Serializer},
    };

    /// Deserialize value wrapped in Some(T)
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        T::deserialize(deserializer).map(Some)
    }

    /// Serialize value if Some(T), unit struct if None
    pub fn serialize<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        if let Some(value) = option {
            value.serialize(serializer)
        } else {
            ().serialize(serializer)
        }
    }
}

/// Ensure no duplicate values exist in a set.
///
/// By default serde has a last-value-wins implementation, if duplicate values for a set exist.
/// Sometimes it is desirable to know when such an event happens, as the first value is overwritten
/// and it can indicate an error in the serialized data.
///
/// This helper returns an error if two identical values exist in a set.
///
/// # Example
///
/// ```rust
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # use std::{collections::HashSet, iter::FromIterator};
/// # #[derive(Debug, Eq, PartialEq)]
/// #[derive(Deserialize)]
/// struct Doc {
///     #[serde(with = "::serde_with::rust::sets_duplicate_value_is_error")]
///     set: HashSet<usize>,
/// }
/// # fn main() {
///
/// // Sets are serialized normally,
/// let s = r#"{"set": [1, 2, 3, 4]}"#;
/// let v = Doc {
///     set: HashSet::from_iter(vec![1, 2, 3, 4]),
/// };
/// assert_eq!(v, serde_json::from_str(s).unwrap());
///
/// // but create an error if duplicate values, like the `1`, exist.
/// let s = r#"{"set": [1, 2, 3, 4, 1]}"#;
/// let res: Result<Doc, _> = serde_json::from_str(s);
/// assert!(res.is_err());
/// # }
/// ```
pub mod sets_duplicate_value_is_error {
    use duplicate_key_impls::PreventDuplicateInsertsSet;
    use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
    use std::{fmt, marker::PhantomData};

    /// Deserialize a set and return an error on duplicate values
    pub fn deserialize<'de, D, T, V>(deserializer: D) -> Result<T, D::Error>
    where
        T: PreventDuplicateInsertsSet<V>,
        V: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        struct SeqVisitor<T, V> {
            marker: PhantomData<T>,
            set_item_type: PhantomData<V>,
        };

        impl<'de, T, V> Visitor<'de> for SeqVisitor<T, V>
        where
            T: PreventDuplicateInsertsSet<V>,
            V: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Self::Value::new(access.size_hint());

                while let Some(value) = access.next_element()? {
                    if !values.insert(value) {
                        return Err(Error::custom("invalid entry: found duplicate value"));
                    };
                }

                Ok(values)
            }
        }

        let visitor = SeqVisitor {
            marker: PhantomData,
            set_item_type: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

/// Ensure no duplicate keys exist in a map.
///
/// By default serde has a last-value-wins implementation, if duplicate keys for a map exist.
/// Sometimes it is desirable to know when such an event happens, as the first value is overwritten
/// and it can indicate an error in the serialized data.
///
/// This helper returns an error if two identical keys exist in a map.
///
/// # Example
///
/// ```rust
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # use std::collections::HashMap;
/// # #[derive(Debug, Eq, PartialEq)]
/// #[derive(Deserialize)]
/// struct Doc {
///     #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
///     map: HashMap<usize, usize>,
/// }
/// # fn main() {
///
/// // Maps are serialized normally,
/// let s = r#"{"map": {"1": 1, "2": 2, "3": 3}}"#;
/// let mut v = Doc {
///     map: HashMap::new(),
/// };
/// v.map.insert(1, 1);
/// v.map.insert(2, 2);
/// v.map.insert(3, 3);
/// assert_eq!(v, serde_json::from_str(s).unwrap());
///
/// // but create an error if duplicate keys, like the `1`, exist.
/// let s = r#"{"map": {"1": 1, "2": 2, "1": 3}}"#;
/// let res: Result<Doc, _> = serde_json::from_str(s);
/// assert!(res.is_err());
/// # }
/// ```
pub mod maps_duplicate_key_is_error {

    use duplicate_key_impls::PreventDuplicateInsertsMap;
    use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
    use std::{fmt, marker::PhantomData};

    /// Deserialize a map and return an error on duplicate keys
    pub fn deserialize<'de, D, T, K, V>(deserializer: D) -> Result<T, D::Error>
    where
        T: PreventDuplicateInsertsMap<K, V>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        struct MapVisitor<T, K, V> {
            marker: PhantomData<T>,
            map_key_type: PhantomData<K>,
            map_value_type: PhantomData<V>,
        };

        impl<'de, T, K, V> Visitor<'de> for MapVisitor<T, K, V>
        where
            T: PreventDuplicateInsertsMap<K, V>,
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = Self::Value::new(access.size_hint());

                while let Some((key, value)) = access.next_entry()? {
                    if !values.insert(key, value) {
                        return Err(Error::custom("invalid entry: found duplicate key"));
                    };
                }

                Ok(values)
            }
        }

        let visitor = MapVisitor {
            marker: PhantomData,
            map_key_type: PhantomData,
            map_value_type: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}

/// Ensure that the first value is taken, if duplicate values exist
///
/// By default serde has a last-value-wins implementation, if duplicate keys for a set exist.
/// Sometimes the opposite strategy is desired. This helper implements a first-value-wins strategy.
pub mod sets_first_value_wins {
    use duplicate_key_impls::DuplicateInsertsFirstWinsSet;
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use std::{fmt, marker::PhantomData};

    /// Deserialize a set and return an error on duplicate values
    pub fn deserialize<'de, D, T, V>(deserializer: D) -> Result<T, D::Error>
    where
        T: DuplicateInsertsFirstWinsSet<V>,
        V: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        struct SeqVisitor<T, V> {
            marker: PhantomData<T>,
            set_item_type: PhantomData<V>,
        };

        impl<'de, T, V> Visitor<'de> for SeqVisitor<T, V>
        where
            T: DuplicateInsertsFirstWinsSet<V>,
            V: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            #[inline]
            fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Self::Value::new(access.size_hint());

                while let Some(value) = access.next_element()? {
                    values.insert(value);
                }

                Ok(values)
            }
        }

        let visitor = SeqVisitor {
            marker: PhantomData,
            set_item_type: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

/// Ensure that the first key is taken, if duplicate keys exist
///
/// By default serde has a last-key-wins implementation, if duplicate keys for a map exist.
/// Sometimes the opposite strategy is desired. This helper implements a first-key-wins strategy.
///
/// # Example
///
/// ```rust
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate serde_with;
/// # use std::collections::HashMap;
/// # #[derive(Debug, Eq, PartialEq)]
/// #[derive(Deserialize)]
/// struct Doc {
///     #[serde(with = "::serde_with::rust::maps_first_key_wins")]
///     map: HashMap<usize, usize>,
/// }
/// # fn main() {
///
/// // Maps are serialized normally,
/// let s = r#"{"map": {"1": 1, "2": 2, "3": 3}}"#;
/// let mut v = Doc {
///     map: HashMap::new(),
/// };
/// v.map.insert(1, 1);
/// v.map.insert(2, 2);
/// v.map.insert(3, 3);
/// assert_eq!(v, serde_json::from_str(s).unwrap());
///
/// // but create an error if duplicate keys, like the `1`, exist.
/// let s = r#"{"map": {"1": 1, "2": 2, "1": 3}}"#;
/// let mut v = Doc {
///     map: HashMap::new(),
/// };
/// v.map.insert(1, 1);
/// v.map.insert(2, 2);
/// assert_eq!(v, serde_json::from_str(s).unwrap());
/// # }
/// ```
pub mod maps_first_key_wins {

    use duplicate_key_impls::DuplicateInsertsFirstWinsMap;
    use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
    use std::{fmt, marker::PhantomData};

    /// Deserialize a map and return an error on duplicate keys
    pub fn deserialize<'de, D, T, K, V>(deserializer: D) -> Result<T, D::Error>
    where
        T: DuplicateInsertsFirstWinsMap<K, V>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        struct MapVisitor<T, K, V> {
            marker: PhantomData<T>,
            map_key_type: PhantomData<K>,
            map_value_type: PhantomData<V>,
        };

        impl<'de, T, K, V> Visitor<'de> for MapVisitor<T, K, V>
        where
            T: DuplicateInsertsFirstWinsMap<K, V>,
            K: Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = Self::Value::new(access.size_hint());

                while let Some((key, value)) = access.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values)
            }
        }

        let visitor = MapVisitor {
            marker: PhantomData,
            map_key_type: PhantomData,
            map_value_type: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}
