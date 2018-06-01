//! De/Serialization of Option Types

/// Serialize inner value if Some(..). If none, serialize the unit struct ().
/// When used with: (skip_serializing_if = "Option::is_none") and 
/// serde(default), you can skip a value if it's None, or serialize its inner
/// value if Some(T).
pub mod unwrap_or_skip {
    use serde::de::{DeserializeOwned, Deserializer};
    use serde::ser::{Serialize, Serializer};

    /// Deserialize value wrapped in Some(T)
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        T::deserialize(deserializer).map(|x| Some(x))
    }

    /// Serialize value if Some(T), unit struct if None
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
