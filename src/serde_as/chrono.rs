use crate::serde_as::*;
use chrono_crate::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl SerializeAs<NaiveDateTime> for DateTime<Utc> {
    fn serialize_as<S>(source: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = DateTime::<Utc>::from_utc(*source, Utc);
        datetime.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, NaiveDateTime> for DateTime<Utc> {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        DateTime::<Utc>::deserialize(deserializer).map(|datetime| datetime.naive_utc())
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
            #[serde(with = "As::<DateTime<Utc>>")]
            stamp: NaiveDateTime,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_crate_opt() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(with = "As::<Option<DateTime<Utc>>>")]
            stamp: Option<NaiveDateTime>,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            })
            .unwrap(),
            "{\"stamp\":\"1994-11-05T08:15:30Z\"}"
        );

        assert_eq!(
            SomeTime {
                stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").ok()
            },
            serde_json::from_str("{\"stamp\":\"1994-11-05T08:15:30Z\"}").unwrap(),
        );
    }

    #[test]
    fn chrono_crate_opt_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct SomeTime {
            #[serde(with = "As::<Vec<Option<DateTime<Utc>>>>")]
            stamps: Vec<Option<NaiveDateTime>>,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: vec![
                    NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
                ],
            })
            .unwrap(),
            "{\"stamps\":[\"1994-11-05T08:15:30Z\",\"1994-11-05T08:15:31Z\"]}"
        );

        assert_eq!(
            SomeTime {
                stamps: vec![
                    NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                    NaiveDateTime::from_str("1994-11-05T08:15:31").ok()
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
            #[serde(with = "As::<BTreeMap<SameAs<i32>, DateTime<Utc>>>")]
            stamps: BTreeMap<i32, NaiveDateTime>,
        }

        assert_eq!(
            serde_json::to_string(&SomeTime {
                stamps: [
                    (1, NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()),
                    (2, NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()),
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
                    (1, NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()),
                    (2, NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()),
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
