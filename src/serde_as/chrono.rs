use crate::serde_as::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl SerializeAs<chrono_crate::NaiveDateTime> for chrono_crate::DateTime<chrono_crate::Utc> {
    fn serialize_as<S>(
        source: &chrono_crate::NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime =
            chrono_crate::DateTime::<chrono_crate::Utc>::from_utc(*source, chrono_crate::Utc);
        datetime.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, chrono_crate::NaiveDateTime>
    for chrono_crate::DateTime<chrono_crate::Utc>
{
    fn deserialize_as<D>(deserializer: D) -> Result<chrono_crate::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        chrono_crate::DateTime::<chrono_crate::Utc>::deserialize(deserializer)
            .map(|datetime| datetime.naive_utc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, str::FromStr};

    #[test]
    fn chrono_crate() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<chrono_crate::DateTime<chrono_crate::Utc>>::serialize_as",
                deserialize_with = "<chrono_crate::DateTime<chrono_crate::Utc>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "chrono_crate::DateTime<chrono_crate::Utc>")]
            stamp: chrono_crate::NaiveDateTime,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_crate_opt() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<Option<chrono_crate::DateTime<chrono_crate::Utc>>>::serialize_as",
                deserialize_with = "<Option<chrono_crate::DateTime<chrono_crate::Utc>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Option<chrono_crate::DateTime<chrono_crate::Utc>>")]
            stamp: Option<chrono_crate::NaiveDateTime>,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_crate_opt_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<Vec<Option<chrono_crate::DateTime<chrono_crate::Utc>>>>::serialize_as",
                deserialize_with = "<Vec<Option<chrono_crate::DateTime<chrono_crate::Utc>>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "Vec<Option<chrono_crate::DateTime<chrono_crate::Utc>>>")]
            stamps: Vec<Option<chrono_crate::NaiveDateTime>>,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: vec![
                    chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
                ],
            })
            .unwrap(),
            "{\"stamps\":[\"1994-11-05T08:15:30Z\",\"1994-11-05T08:15:31Z\"]}"
        );

        assert_eq!(
            SomeTime {
                stamps: vec![
                    chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
                ],
            },
            serde_json::from_str(
                "{\"stamps\":[\"1994-11-05T08:15:30Z\",\"1994-11-05T08:15:31Z\"]}"
            )
            .unwrap(),
        );
    }

    #[test]
    fn chrono_crate_hash_map() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(
                serialize_with = "<BTreeMap<SameAs<i32>, chrono_crate::DateTime<chrono_crate::Utc>>>::serialize_as",
                deserialize_with = "<BTreeMap<SameAs<i32>, chrono_crate::DateTime<chrono_crate::Utc>>>::deserialize_as"
            )]
            // FIXME: #[serde(as = "HashMap<SameAs<i32>, chrono_crate::DateTime<chrono_crate::Utc>>")]
            stamps: BTreeMap<i32, chrono_crate::NaiveDateTime>,
        }

        // FIXME: this test is flaky - random in hash-map sequence
        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: [
                    (
                        1,
                        chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
                    ),
                    (
                        2,
                        chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            })
            .unwrap(),
            "{\"stamps\":{\"1\":\"1994-11-05T08:15:30Z\",\"2\":\"1994-11-05T08:15:31Z\"}}"
        );

        assert_eq!(
            SomeTime {
                stamps: [
                    (
                        1,
                        chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
                    ),
                    (
                        2,
                        chrono_crate::NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            serde_json::from_str(
                "{\"stamps\":{\"1\":\"1994-11-05T08:15:30Z\",\"2\":\"1994-11-05T08:15:31Z\"}}"
            )
            .unwrap(),
        );
    }
}
