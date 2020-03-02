#![cfg(feature = "chrono")]

mod utils;

use crate::utils::is_equal;
use chrono_crate::{DateTime, NaiveDateTime, Utc};
use pretty_assertions::assert_eq;
use serde::Serialize;
use serde_derive::Deserialize;
use serde_with::{As, SameAs};
use std::{collections::BTreeMap, str::FromStr};

fn new_datetime(secs: i64, nsecs: u32) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
}

#[test]
fn json_datetime_from_any_to_string_deserialization() {
    #[derive(Debug, Deserialize)]
    struct S {
        #[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")]
        date: DateTime<Utc>,
    }
    let from = r#"[
        { "date": 1478563200 },
        { "date": 0 },
        { "date": -86000 },
        { "date": 1478563200.123 },
        { "date": 0.000 },
        { "date": -86000.999 },
        { "date": "1478563200.123" },
        { "date": "0.000" },
        { "date": "-86000.999" }
    ]"#;

    let res: Vec<S> = serde_json::from_str(from).unwrap();

    // just integers
    assert_eq!(new_datetime(1_478_563_200, 0), res[0].date);
    assert_eq!(new_datetime(0, 0), res[1].date);
    assert_eq!(new_datetime(-86000, 0), res[2].date);

    // floats, shows precision errors in subsecond part
    assert_eq!(new_datetime(1_478_563_200, 122_999_906), res[3].date);
    assert_eq!(new_datetime(0, 0), res[4].date);
    assert_eq!(new_datetime(-86000, 998_999_999), res[5].date);

    // string representation of floats
    assert_eq!(new_datetime(1_478_563_200, 123_000_000), res[6].date);
    assert_eq!(new_datetime(0, 0), res[7].date);
    assert_eq!(new_datetime(-86000, 999_000_000), res[8].date);
}

#[test]
fn test_chrono_naive_date_time() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde(with = "As::<DateTime<Utc>>")]
        stamp: NaiveDateTime,
    }
    is_equal(
        SomeTime {
            stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap(),
        },
        r#"{"stamp":"1994-11-05T08:15:30Z"}"#,
    );
}
#[test]
fn test_chrono_option_naive_date_time() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde(with = "As::<Option<DateTime<Utc>>>")]
        stamp: Option<NaiveDateTime>,
    }
    is_equal(
        SomeTime {
            stamp: NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
        },
        r#"{"stamp":"1994-11-05T08:15:30Z"}"#,
    );
}
#[test]
fn test_chrono_vec_option_naive_date_time() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde(with = "As::<Vec<Option<DateTime<Utc>>>>")]
        stamps: Vec<Option<NaiveDateTime>>,
    }
    is_equal(
        SomeTime {
            stamps: vec![
                NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
                NaiveDateTime::from_str("1994-11-05T08:15:31").ok(),
            ],
        },
        r#"{"stamps":["1994-11-05T08:15:30Z","1994-11-05T08:15:31Z"]}"#,
    );
}
#[test]
fn test_chrono_btree_map_naive_date_time() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde(with = "As::<BTreeMap<SameAs<i32>, DateTime<Utc>>>")]
        stamps: BTreeMap<i32, NaiveDateTime>,
    }
    is_equal(
        SomeTime {
            stamps: vec![
                (1, NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()),
                (2, NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()),
            ]
            .into_iter()
            .collect(),
        },
        r#"{"stamps":{"1":"1994-11-05T08:15:30Z","2":"1994-11-05T08:15:31Z"}}"#,
    );
}
