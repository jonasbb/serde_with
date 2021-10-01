//!TODO

use crate::{formats, DeserializeAs, SerializeAs};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::marker::PhantomData;

/// todo

// The padding might be better as `const PADDING: bool = true`
// https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html#featureconst_generics_default/
#[derive(Copy, Clone, Debug, Default)]
pub struct Base64<CHARSET: CharacterSet = Standard, PADDING: formats::Format = formats::Padded>(
    PhantomData<(CHARSET, PADDING)>,
);

impl<T, CHARSET> SerializeAs<T> for Base64<CHARSET, formats::Padded>
where
    T: AsRef<[u8]>,
    CHARSET: CharacterSet,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        base64_crate::encode_config(source, base64_crate::Config::new(CHARSET::charset(), true))
            .serialize(serializer)
    }
}

impl<T, CHARSET> SerializeAs<T> for Base64<CHARSET, formats::Unpadded>
where
    T: AsRef<[u8]>,
    CHARSET: CharacterSet,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        base64_crate::encode_config(source, base64_crate::Config::new(CHARSET::charset(), false))
            .serialize(serializer)
    }
}

impl<'de, T, CHARSET, FORMAT> DeserializeAs<'de, T> for Base64<CHARSET, FORMAT>
where
    T: TryFrom<Vec<u8>>,
    CHARSET: CharacterSet,
    FORMAT: formats::Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| {
                base64_crate::decode_config(
                    &*s,
                    base64_crate::Config::new(CHARSET::charset(), false),
                )
                .map_err(Error::custom)
            })
            .and_then(|vec: Vec<u8>| {
                let length = vec.len();
                vec.try_into().map_err(|_e: T::Error| {
                    Error::custom(format!(
                        "Can't convert a Byte Vector of length {} to the output type.",
                        length
                    ))
                })
            })
    }
}

/// A base64 character set from [this list](base64_crate::CharacterSet).
pub trait CharacterSet {
    /// todo
    fn charset() -> base64_crate::CharacterSet;
}

/// The standard character set (uses `+` and `/`).
///
/// See [RFC 3548](https://tools.ietf.org/html/rfc3548#section-3).
#[derive(Copy, Clone, Debug, Default)]
pub struct Standard;
impl CharacterSet for Standard {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::Standard
    }
}

/// The URL safe character set (uses `-` and `_`).
///
/// See [RFC 3548](https://tools.ietf.org/html/rfc3548#section-3).
#[derive(Copy, Clone, Debug, Default)]
pub struct UrlSafe;
impl CharacterSet for UrlSafe {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::UrlSafe
    }
}

/// The `crypt(3)` character set (uses `./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`).
///
/// Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses.
#[derive(Copy, Clone, Debug, Default)]
pub struct Crypt;
impl CharacterSet for Crypt {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::Crypt
    }
}

/// The bcrypt character set (uses `./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`).
#[derive(Copy, Clone, Debug, Default)]
pub struct Bcrypt;
impl CharacterSet for Bcrypt {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::Bcrypt
    }
}

/// The character set used in IMAP-modified UTF-7 (uses `+` and `,`).
///
/// See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3).
#[derive(Copy, Clone, Debug, Default)]
pub struct ImapMutf7;
impl CharacterSet for ImapMutf7 {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::ImapMutf7
    }
}

/// The character set used in BinHex 4.0 files.
///
/// See [BinHex 4.0 Definition](http://files.stairways.com/other/binhex-40-specs-info.txt).
#[derive(Copy, Clone, Debug, Default)]
pub struct BinHex;
impl CharacterSet for BinHex {
    fn charset() -> base64_crate::CharacterSet {
        base64_crate::CharacterSet::BinHex
    }
}
