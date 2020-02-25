#[cfg(test)]
use super::tests::is_equal;
use super::*;
use serde::de::DeserializeOwned;

#[derive(Copy, Clone, Debug, Default)]
pub struct JsonString;

impl<T> SerializeAs<T> for JsonString
where
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        crate::json::nested::serialize(source, serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for JsonString
where
    T: DeserializeOwned,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        crate::json::nested::deserialize(deserializer)
    }
}

#[test]
fn test_nested_json() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<JsonString>")]
        value: Nested,
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        #[serde(with = "As::<DisplayFromStr>")]
        value: u32,
    }

    is_equal(
        Struct {
            value: Nested { value: 444 },
        },
        r#"{"value":"{\"value\":\"444\"}"}"#,
    );
}
