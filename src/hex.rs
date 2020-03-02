use crate::{de::DeserializeAs, ser::SerializeAs};
use serde::{Deserialize, Deserializer, Serializer};

#[derive(Copy, Clone, Debug, Default)]
pub struct Hex;

// TODO: AsRef
impl SerializeAs<Vec<u8>> for Hex {
    fn serialize_as<S>(source: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&::hex::encode(source))
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for Hex {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FIXME: map decode errors
        <&'de str as Deserialize<'de>>::deserialize(deserializer).map(|s| ::hex::decode(s).unwrap())
    }
}
