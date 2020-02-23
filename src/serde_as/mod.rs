#![allow(missing_debug_implementations, missing_docs)]

pub use self::{de::DeserializeAs, ser::SerializeAs};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

#[cfg(feature = "chrono")]
mod chrono;
pub mod de;
#[cfg(feature = "hex")]
mod hex;
pub mod ser;
#[cfg(test)]
mod tests;
mod utils;

// TODO: doc
pub struct SameAs<T>(PhantomData<T>);

#[derive(Copy, Clone, Debug, Default)]
pub struct Same;

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
pub struct DisplayString;
