use super::*;

pub(in crate::serde_as) mod impls;

pub trait SerializeAs<T> {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
