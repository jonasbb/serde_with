//! De/Serialization of Option Types

/// Serialize value if Some(..), skip if None
pub mod unwrap_or_skip {
    use serde::de::{DeserializeOwned, Deserializer, Error, Visitor};
    use serde::ser::{self, Serialize, Serializer};
    use serde_json;
    use std::fmt;
    use std::marker::PhantomData;

    /// Deserialize value wrapped in Some(T)
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        deserializer.deserialize(Some(T))
    }

    /// Serialize value if Some(T), skip if None
    pub fn serialize<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        if let &Some(ref value) = option {
            value.serialize(serializer)
        } else {
            ().serialize(serializer)
        }
    }
}
