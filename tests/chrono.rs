#![cfg(feature = "chrono")]

mod utils;

use crate::utils::{
    check_deserialization, check_error_deserialization, check_serialization, is_equal,
};
use chrono_crate::{DateTime, Duration, NaiveDateTime, Utc};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds, DurationSecondsWithFrac, Flexible, Integer, SameAs};
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
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde_as(as = "DateTime<Utc>")]
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
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde_as(as = "Option<DateTime<Utc>>")]
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
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde_as(as = "Vec<Option<DateTime<Utc>>>")]
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
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeTime {
        #[serde_as(as = "BTreeMap<SameAs<i32>, DateTime<Utc>>")]
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

#[test]
fn test_chrono_duration_seconds() {
    let zero = Duration::zero();
    let one_second = Duration::seconds(1);
    // let third_second = Duration::nanoseconds(333_333_333);
    let half_second = Duration::nanoseconds(500_000_000);
    // let one_and_third_second = one_second + third_second;
    let minus_one_second = zero - one_second;
    let minus_half_second = zero - half_second;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntStrict {
        #[serde_as(as = "DurationSeconds")]
        value: Duration,
    };

    is_equal(StructIntStrict { value: zero }, r#"{"value":0}"#);
    is_equal(StructIntStrict { value: one_second }, r#"{"value":1}"#);
    is_equal(
        StructIntStrict {
            value: minus_one_second,
        },
        r#"{"value":-1}"#,
    );
    check_serialization(StructIntStrict { value: half_second }, r#"{"value":1}"#);
    check_serialization(
        StructIntStrict {
            value: minus_half_second,
        },
        r#"{"value":-1}"#,
    );
    check_error_deserialization::<StructIntStrict>(
        r#"{"value":"1"}"#,
        r#"invalid type: string "1", expected i64 at line 1 column 12"#,
    );
    check_error_deserialization::<StructIntStrict>(
        r#"{"value":9223372036854775808}"#,
        r#"invalid value: integer `9223372036854775808`, expected i64 at line 1 column 28"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntFlexible {
        #[serde_as(as = "DurationSeconds<Integer, Flexible>")]
        value: Duration,
    };

    is_equal(StructIntFlexible { value: zero }, r#"{"value":0}"#);
    is_equal(StructIntFlexible { value: one_second }, r#"{"value":1}"#);
    check_serialization(StructIntFlexible { value: half_second }, r#"{"value":1}"#);
    check_serialization(
        StructIntFlexible {
            value: minus_half_second,
        },
        r#"{"value":-1}"#,
    );
    check_deserialization(
        StructIntFlexible { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    check_deserialization(
        StructIntFlexible {
            value: minus_half_second,
        },
        r#"{"value":"-0.5"}"#,
    );
    check_deserialization(StructIntFlexible { value: one_second }, r#"{"value":"1"}"#);
    check_deserialization(
        StructIntFlexible {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_deserialization(StructIntFlexible { value: zero }, r#"{"value":"0"}"#);
    check_error_deserialization::<StructIntFlexible>(
        r#"{"value":"a"}"#,
        r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 12"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict {
        #[serde_as(as = "DurationSeconds<f64>")]
        value: Duration,
    };

    is_equal(Structf64Strict { value: zero }, r#"{"value":0.0}"#);
    is_equal(Structf64Strict { value: one_second }, r#"{"value":1.0}"#);
    is_equal(
        Structf64Strict {
            value: minus_one_second,
        },
        r#"{"value":-1.0}"#,
    );
    check_serialization(Structf64Strict { value: half_second }, r#"{"value":1.0}"#);
    check_serialization(
        Structf64Strict {
            value: minus_half_second,
        },
        r#"{"value":-1.0}"#,
    );
    check_deserialization(Structf64Strict { value: half_second }, r#"{"value":0.5}"#);
    check_deserialization(
        Structf64Strict {
            value: minus_half_second,
        },
        r#"{"value":-0.5}"#,
    );
    check_error_deserialization::<Structf64Strict>(
        r#"{"value":"1"}"#,
        r#"invalid type: string "1", expected f64 at line 1 column 12"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible {
        #[serde_as(as = "DurationSeconds<f64, Flexible>")]
        value: Duration,
    };

    is_equal(Structf64Flexible { value: zero }, r#"{"value":0.0}"#);
    is_equal(Structf64Flexible { value: one_second }, r#"{"value":1.0}"#);
    is_equal(
        Structf64Flexible {
            value: minus_one_second,
        },
        r#"{"value":-1.0}"#,
    );
    check_serialization(Structf64Flexible { value: half_second }, r#"{"value":1.0}"#);
    check_serialization(
        Structf64Flexible {
            value: minus_half_second,
        },
        r#"{"value":-1.0}"#,
    );
    check_deserialization(
        Structf64Flexible { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    check_deserialization(
        Structf64Flexible {
            value: minus_half_second,
        },
        r#"{"value":"-0.5"}"#,
    );
    check_deserialization(Structf64Flexible { value: one_second }, r#"{"value":"1"}"#);
    check_deserialization(
        Structf64Flexible {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_deserialization(Structf64Flexible { value: zero }, r#"{"value":"0"}"#);
    check_error_deserialization::<Structf64Flexible>(
        r#"{"value":"a"}"#,
        r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 12"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict {
        #[serde_as(as = "DurationSeconds<String>")]
        value: Duration,
    };

    is_equal(StructStringStrict { value: zero }, r#"{"value":"0"}"#);
    is_equal(StructStringStrict { value: one_second }, r#"{"value":"1"}"#);
    is_equal(
        StructStringStrict {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_serialization(
        StructStringStrict { value: half_second },
        r#"{"value":"1"}"#,
    );
    check_serialization(
        StructStringStrict {
            value: minus_half_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_error_deserialization::<StructStringStrict>(
        r#"{"value":1}"#,
        // TODO the error message should not talk about "json object"
        r#"invalid type: integer `1`, expected valid json object at line 1 column 10"#,
    );
    check_error_deserialization::<StructStringStrict>(
        r#"{"value":-1}"#,
        r#"invalid type: integer `-1`, expected valid json object at line 1 column 11"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible {
        #[serde_as(as = "DurationSeconds<String, Flexible>")]
        value: Duration,
    };

    is_equal(StructStringFlexible { value: zero }, r#"{"value":"0"}"#);
    is_equal(
        StructStringFlexible { value: one_second },
        r#"{"value":"1"}"#,
    );
    is_equal(
        StructStringFlexible {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_serialization(
        StructStringFlexible { value: half_second },
        r#"{"value":"1"}"#,
    );
    check_deserialization(
        StructStringFlexible { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    check_deserialization(
        StructStringFlexible { value: one_second },
        r#"{"value":"1"}"#,
    );
    check_deserialization(StructStringFlexible { value: zero }, r#"{"value":"0"}"#);
    check_error_deserialization::<StructStringFlexible>(
        r#"{"value":"a"}"#,
        r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 12"#,
    );
}

#[test]
fn test_chrono_duration_seconds_with_frac() {
    let zero = Duration::zero();
    let one_second = Duration::seconds(1);
    // let third_second = Duration::nanoseconds(333_333_333);
    let half_second = Duration::nanoseconds(500_000_000);
    // let one_and_third_second = one_second + third_second;
    let minus_one_second = zero - one_second;
    let minus_half_second = zero - half_second;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict {
        #[serde_as(as = "DurationSecondsWithFrac<f64>")]
        value: Duration,
    };

    is_equal(Structf64Strict { value: zero }, r#"{"value":0.0}"#);
    is_equal(Structf64Strict { value: one_second }, r#"{"value":1.0}"#);
    is_equal(
        Structf64Strict {
            value: minus_one_second,
        },
        r#"{"value":-1.0}"#,
    );
    is_equal(Structf64Strict { value: half_second }, r#"{"value":0.5}"#);
    is_equal(
        Structf64Strict {
            value: minus_half_second,
        },
        r#"{"value":-0.5}"#,
    );
    check_error_deserialization::<Structf64Strict>(
        r#"{"value":"1"}"#,
        r#"invalid type: string "1", expected f64 at line 1 column 12"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible {
        #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
        value: Duration,
    };

    is_equal(Structf64Flexible { value: zero }, r#"{"value":0.0}"#);
    is_equal(Structf64Flexible { value: one_second }, r#"{"value":1.0}"#);
    is_equal(
        Structf64Flexible {
            value: minus_one_second,
        },
        r#"{"value":-1.0}"#,
    );
    is_equal(
        Structf64Flexible {
            value: minus_half_second,
        },
        r#"{"value":-0.5}"#,
    );
    check_deserialization(Structf64Flexible { value: one_second }, r#"{"value":"1"}"#);
    check_deserialization(
        Structf64Flexible {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    check_deserialization(
        Structf64Flexible { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    check_deserialization(Structf64Flexible { value: zero }, r#"{"value":"0"}"#);
    check_error_deserialization::<Structf64Flexible>(
        r#"{"value":"a"}"#,
        r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 12"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict {
        #[serde_as(as = "DurationSecondsWithFrac<String>")]
        value: Duration,
    };

    is_equal(StructStringStrict { value: zero }, r#"{"value":"0"}"#);
    is_equal(StructStringStrict { value: one_second }, r#"{"value":"1"}"#);
    is_equal(
        StructStringStrict {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    is_equal(
        StructStringStrict { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    is_equal(
        StructStringStrict {
            value: minus_half_second,
        },
        r#"{"value":"-0.5"}"#,
    );
    check_error_deserialization::<StructStringStrict>(
        r#"{"value":1}"#,
        r#"invalid type: integer `1`, expected a string at line 1 column 10"#,
    );
    check_error_deserialization::<StructStringStrict>(
        r#"{"value":-1}"#,
        r#"invalid type: integer `-1`, expected a string at line 1 column 11"#,
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible {
        #[serde_as(as = "DurationSecondsWithFrac<String, Flexible>")]
        value: Duration,
    };

    is_equal(StructStringFlexible { value: zero }, r#"{"value":"0"}"#);
    is_equal(
        StructStringFlexible { value: one_second },
        r#"{"value":"1"}"#,
    );
    is_equal(
        StructStringFlexible {
            value: minus_one_second,
        },
        r#"{"value":"-1"}"#,
    );
    is_equal(
        StructStringFlexible { value: half_second },
        r#"{"value":"0.5"}"#,
    );
    check_deserialization(
        StructStringFlexible { value: one_second },
        r#"{"value":"1"}"#,
    );
    check_deserialization(StructStringFlexible { value: zero }, r#"{"value":"0"}"#);
    check_error_deserialization::<StructStringFlexible>(
        r#"{"value":"a"}"#,
        r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 12"#,
    );
}
