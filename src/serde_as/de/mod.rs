use super::*;

pub(in crate::serde_as) mod impls;

pub trait DeserializeAs<'de, T>: Sized {
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>;

    // TODO: deserialize_as_into
}
