mod utils;

use crate::utils::{
    check_deserialization, check_error_deserialization, check_serialization, is_equal,
};
use chrono_crate::{DateTime, Duration, NaiveDateTime, Utc};
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::{
    formats::Flexible, serde_as, DurationSeconds, DurationSecondsWithFrac, TimestampSeconds,
    TimestampSecondsWithFrac,
};
use std::{collections::BTreeMap, iter::FromIterator, str::FromStr};

fn new_datetime(secs: i64, nsecs: u32) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
}

#[test]
fn json_datetime_from_any_to_string_deserialization() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct S(#[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")] DateTime<Utc>);

    // just integers
    check_deserialization(
        vec![
            S(new_datetime(1_478_563_200, 0)),
            S(new_datetime(0, 0)),
            S(new_datetime(-86000, 0)),
        ],
        r#"[
            1478563200,
            0,
            -86000
        ]"#,
    );

    // floats, shows precision errors in subsecond part
    check_deserialization(
        vec![
            S(new_datetime(1_478_563_200, 122_999_906)),
            S(new_datetime(0, 0)),
            S(new_datetime(-86000, 998_999_999)),
        ],
        r#"[
            1478563200.123,
            0.000,
            -86000.999
        ]"#,
    );

    // string representation of floats
    check_deserialization(
        vec![
            S(new_datetime(1_478_563_200, 123_000_000)),
            S(new_datetime(0, 0)),
            S(new_datetime(-86000, 999_000_000)),
        ],
        r#"[
            "1478563200.123",
            "0.000",
            "-86000.999"
        ]"#,
    );
}

#[test]
fn test_chrono_naive_date_time() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct S(#[serde_as(as = "DateTime<Utc>")] NaiveDateTime);

    is_equal(
        S(NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()),
        expect![[r#""1994-11-05T08:15:30Z""#]],
    );
}

#[test]
fn test_chrono_option_naive_date_time() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct S(#[serde_as(as = "Option<DateTime<Utc>>")] Option<NaiveDateTime>);

    is_equal(
        S(NaiveDateTime::from_str("1994-11-05T08:15:30").ok()),
        expect![[r#""1994-11-05T08:15:30Z""#]],
    );
}

#[test]
fn test_chrono_vec_option_naive_date_time() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct S(#[serde_as(as = "Vec<Option<DateTime<Utc>>>")] Vec<Option<NaiveDateTime>>);

    is_equal(
        S(vec![
            NaiveDateTime::from_str("1994-11-05T08:15:30").ok(),
            NaiveDateTime::from_str("1994-11-05T08:15:31").ok(),
        ]),
        expect![[r#"
            [
              "1994-11-05T08:15:30Z",
              "1994-11-05T08:15:31Z"
            ]"#]],
    );
}

#[test]
fn test_chrono_btreemap_naive_date_time() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct S(#[serde_as(as = "BTreeMap<_, DateTime<Utc>>")] BTreeMap<i32, NaiveDateTime>);

    is_equal(
        S(BTreeMap::from_iter(vec![
            (1, NaiveDateTime::from_str("1994-11-05T08:15:30").unwrap()),
            (2, NaiveDateTime::from_str("1994-11-05T08:15:31").unwrap()),
        ])),
        expect![[r#"
            {
              "1": "1994-11-05T08:15:30Z",
              "2": "1994-11-05T08:15:31Z"
            }"#]],
    );
}

#[test]
fn test_chrono_duration_seconds() {
    let zero = Duration::zero();
    let one_second = Duration::seconds(1);
    let half_second = Duration::nanoseconds(500_000_000);
    let minus_one_second = zero - one_second;
    let minus_half_second = zero - half_second;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntStrict(#[serde_as(as = "DurationSeconds<i64>")] Duration);

    is_equal(StructIntStrict(zero), expect![[r#"0"#]]);
    is_equal(StructIntStrict(one_second), expect![[r#"1"#]]);
    is_equal(StructIntStrict(minus_one_second), expect![[r#"-1"#]]);
    check_serialization(StructIntStrict(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntStrict(minus_half_second), expect![[r#"-1"#]]);
    check_error_deserialization::<StructIntStrict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected i64 at line 1 column 3"#]],
    );
    check_error_deserialization::<StructIntStrict>(
        r#"9223372036854775808"#,
        expect![[
            r#"invalid value: integer `9223372036854775808`, expected i64 at line 1 column 19"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntFlexible(#[serde_as(as = "DurationSeconds<i64, Flexible>")] Duration);

    is_equal(StructIntFlexible(zero), expect![[r#"0"#]]);
    is_equal(StructIntFlexible(one_second), expect![[r#"1"#]]);
    check_serialization(StructIntFlexible(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntFlexible(minus_half_second), expect![[r#"-1"#]]);
    check_deserialization(StructIntFlexible(half_second), r#""0.5""#);
    check_deserialization(StructIntFlexible(minus_half_second), r#""-0.5""#);
    check_deserialization(StructIntFlexible(one_second), r#""1""#);
    check_deserialization(StructIntFlexible(minus_one_second), r#""-1""#);
    check_deserialization(StructIntFlexible(zero), r#""0""#);
    check_error_deserialization::<StructIntFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "DurationSeconds<f64>")] Duration);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Strict(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Strict(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Strict(one_second), r#"0.5"#);
    check_deserialization(Structf64Strict(minus_one_second), r#"-0.5"#);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(#[serde_as(as = "DurationSeconds<f64, Flexible>")] Duration);

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Flexible(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Flexible(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Flexible(half_second), r#""0.5""#);
    check_deserialization(Structf64Flexible(minus_half_second), r#""-0.5""#);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(minus_one_second), r#""-1""#);
    check_deserialization(Structf64Flexible(zero), r#""0""#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "DurationSeconds<String>")] Duration);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringStrict(half_second), expect![[r#""1""#]]);
    check_serialization(StructStringStrict(minus_half_second), expect![[r#""-1""#]]);
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        // TODO the error message should not talk about "json object"
        expect![[r#"invalid type: integer `1`, expected valid json object at line 1 column 1"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"-1"#,
        expect![[r#"invalid type: integer `-1`, expected valid json object at line 1 column 2"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(#[serde_as(as = "DurationSeconds<String, Flexible>")] Duration);

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringFlexible(half_second), expect![[r#""1""#]]);
    check_deserialization(StructStringFlexible(half_second), r#""0.5""#);
    check_deserialization(StructStringFlexible(one_second), r#""1""#);
    check_deserialization(StructStringFlexible(zero), r#""0""#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}

#[test]
fn test_chrono_duration_seconds_with_frac() {
    let zero = Duration::zero();
    let one_second = Duration::seconds(1);
    let half_second = Duration::nanoseconds(500_000_000);
    let minus_one_second = zero - one_second;
    let minus_half_second = zero - half_second;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "DurationSecondsWithFrac<f64>")] Duration);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Strict(half_second), expect![[r#"0.5"#]]);
    is_equal(Structf64Strict(minus_half_second), expect![[r#"-0.5"#]]);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(#[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")] Duration);

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Flexible(minus_half_second), expect![[r#"-0.5"#]]);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(minus_one_second), r#""-1""#);
    check_deserialization(Structf64Flexible(half_second), r#""0.5""#);
    check_deserialization(Structf64Flexible(zero), r#""0""#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "DurationSecondsWithFrac<String>")] Duration);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringStrict(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringStrict(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    is_equal(
        StructStringStrict(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 1"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"-1"#,
        expect![[r#"invalid type: integer `-1`, expected a string at line 1 column 2"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(
        #[serde_as(as = "DurationSecondsWithFrac<String, Flexible>")] Duration,
    );

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringFlexible(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringFlexible(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_deserialization(StructStringFlexible(one_second), r#""1""#);
    check_deserialization(StructStringFlexible(zero), r#""0""#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}

#[test]
fn test_chrono_timestamp_seconds() {
    let zero = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
    let one_second = zero + Duration::seconds(1);
    let half_second = zero + Duration::nanoseconds(500_000_000);
    let minus_one_second = zero - Duration::seconds(1);
    let minus_half_second = zero - Duration::nanoseconds(500_000_000);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntStrict(#[serde_as(as = "TimestampSeconds")] DateTime<Utc>);

    is_equal(StructIntStrict(zero), expect![[r#"0"#]]);
    is_equal(StructIntStrict(one_second), expect![[r#"1"#]]);
    is_equal(StructIntStrict(minus_one_second), expect![[r#"-1"#]]);
    check_serialization(StructIntStrict(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntStrict(minus_half_second), expect![[r#"-1"#]]);
    check_error_deserialization::<StructIntStrict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected i64 at line 1 column 3"#]],
    );
    check_error_deserialization::<StructIntStrict>(
        r#"0.123"#,
        expect![[r#"invalid type: floating point `0.123`, expected i64 at line 1 column 5"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntFlexible(#[serde_as(as = "TimestampSeconds<i64, Flexible>")] DateTime<Utc>);

    is_equal(StructIntFlexible(zero), expect![[r#"0"#]]);
    is_equal(StructIntFlexible(one_second), expect![[r#"1"#]]);
    is_equal(StructIntFlexible(minus_one_second), expect![[r#"-1"#]]);
    check_serialization(StructIntFlexible(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntFlexible(minus_half_second), expect![[r#"-1"#]]);
    check_deserialization(StructIntFlexible(one_second), r#""1""#);
    check_deserialization(StructIntFlexible(one_second), r#"1.0"#);
    check_deserialization(StructIntFlexible(minus_half_second), r#""-0.5""#);
    check_deserialization(StructIntFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructIntFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "TimestampSeconds<f64>")] DateTime<Utc>);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Strict(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Strict(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Strict(one_second), r#"0.5"#);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(#[serde_as(as = "TimestampSeconds<f64, Flexible>")] DateTime<Utc>);

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Flexible(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Flexible(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(one_second), r#"1.0"#);
    check_deserialization(Structf64Flexible(minus_half_second), r#""-0.5""#);
    check_deserialization(Structf64Flexible(half_second), r#"0.5"#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "TimestampSeconds<String>")] DateTime<Utc>);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringStrict(half_second), expect![[r#""1""#]]);
    check_serialization(StructStringStrict(minus_half_second), expect![[r#""-1""#]]);
    check_deserialization(StructStringStrict(one_second), r#""1""#);
    check_error_deserialization::<StructStringStrict>(
        r#""0.5""#,
        expect![[r#"invalid digit found in string at line 1 column 5"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#""-0.5""#,
        expect![[r#"invalid digit found in string at line 1 column 6"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected valid json object at line 1 column 1"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"0.0"#,
        expect![[
            r#"invalid type: floating point `0`, expected valid json object at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(
        #[serde_as(as = "TimestampSeconds<String, Flexible>")] DateTime<Utc>,
    );

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringFlexible(half_second), expect![[r#""1""#]]);
    check_serialization(
        StructStringFlexible(minus_half_second),
        expect![[r#""-1""#]],
    );
    check_deserialization(StructStringFlexible(one_second), r#"1"#);
    check_deserialization(StructStringFlexible(one_second), r#"1.0"#);
    check_deserialization(StructStringFlexible(minus_half_second), r#""-0.5""#);
    check_deserialization(StructStringFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}

#[test]
fn test_chrono_timestamp_seconds_with_frac() {
    let zero = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
    let one_second = zero + Duration::seconds(1);
    let half_second = zero + Duration::nanoseconds(500_000_000);
    let minus_one_second = zero - Duration::seconds(1);
    let minus_half_second = zero - Duration::nanoseconds(500_000_000);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "TimestampSecondsWithFrac<f64>")] DateTime<Utc>);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Strict(half_second), expect![[r#"0.5"#]]);
    is_equal(Structf64Strict(minus_half_second), expect![[r#"-0.5"#]]);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(
        #[serde_as(as = "TimestampSecondsWithFrac<f64, Flexible>")] DateTime<Utc>,
    );

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Flexible(half_second), expect![[r#"0.5"#]]);
    is_equal(Structf64Flexible(minus_half_second), expect![[r#"-0.5"#]]);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(minus_half_second), r#""-0.5""#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "TimestampSecondsWithFrac<String>")] DateTime<Utc>);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringStrict(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringStrict(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 1"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"0.0"#,
        expect![[r#"invalid type: floating point `0`, expected a string at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(
        #[serde_as(as = "TimestampSecondsWithFrac<String, Flexible>")] DateTime<Utc>,
    );

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringFlexible(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringFlexible(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_deserialization(StructStringFlexible(one_second), r#"1"#);
    check_deserialization(StructStringFlexible(one_second), r#"1.0"#);
    check_deserialization(StructStringFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}
