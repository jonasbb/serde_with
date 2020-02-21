pub struct Hex;

// TODO: AsRef
impl SerializeAs<Vec<u8>> for Hex {
    fn serialize_as<S>(source: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // FIXME: optimize
        serializer.serialize_str(&hex::encode(source))
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for Hex {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FIXME: map decode errors
        <&'de str as Deserialize<'de>>::deserialize(deserializer).map(|s| hex::decode(s).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hex_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeBytes {
            #[serde(
                serialize_with = "<Vec<Hex>>::serialize_as",
                deserialize_with = "<Vec<Hex>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Vec<Hex>")]
            bytes: Vec<Vec<u8>>,
        }

        assert_eq!(
            serde_json::to_string(&SomeBytes {
                bytes: vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]]
            })
            .unwrap(),
            "{\"bytes\":[\"00010203\",\"04050607\"]}"
        );

        assert_eq!(
            SomeBytes {
                bytes: vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]]
            },
            serde_json::from_str("{\"bytes\":[\"00010203\",\"04050607\"]}").unwrap(),
        );
    }
}
