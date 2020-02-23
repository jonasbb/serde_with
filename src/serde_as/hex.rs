use super::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeBytes {
            #[serde(with = "As::<Vec<Hex>>")]
            bytes: Vec<Vec<u8>>,
        }

        assert_eq!(
            serde_json::to_string(&SomeBytes {
                bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]]
            })
            .unwrap(),
            "{\"bytes\":[\"0001020d\",\"0e050607\"]}"
        );

        assert_eq!(
            SomeBytes {
                bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]]
            },
            serde_json::from_str("{\"bytes\":[\"0001020d\",\"0e050607\"]}").unwrap(),
        );
    }
}
