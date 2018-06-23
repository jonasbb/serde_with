#![cfg(feature = "chrono")]

extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_with;
#[macro_use]
extern crate pretty_assertions;

use chrono::{DateTime, NaiveDateTime, Utc};

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

use serde_with::chrono::*;
#[test]
fn test_ts_s_ms_float_deserialize() {
    #[derive(Eq, PartialEq, Debug, Deserialize)]
    struct S {
        #[serde(with = "TsSecondsWithMsAsFloat")]
        ts: Timestamp,
    }
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 0,
    };
    assert_eq!(S { ts }, serde_json::from_str(r#"{"ts": 1.0}"#).unwrap());
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 538_000_000,
    };
    assert_eq!(S { ts }, serde_json::from_str(r#"{"ts": 1.538}"#).unwrap());
    let ts = Timestamp {
        seconds: -1,
        nanoseconds: 500_000_000,
    };
    assert_eq!(S { ts }, serde_json::from_str(r#"{"ts": -1.5}"#).unwrap());
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 538_000_000,
    };
    assert_eq!(
        S { ts },
        serde_json::from_str(r#"{"ts": 1.538123}"#).unwrap()
    );
}

#[test]
fn test_ts_s_ms_float_serialize() {
    #[derive(Serialize)]
    struct S {
        #[serde(with = "TsSecondsWithMsAsFloat")]
        ts: Timestamp,
    }
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 0,
    };
    assert_eq!(r#"{"ts":1.0}"#, serde_json::to_string(&S { ts }).unwrap());
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 538_000_000,
    };
    assert_eq!(r#"{"ts":1.538}"#, serde_json::to_string(&S { ts }).unwrap());
    let ts = Timestamp {
        seconds: 1,
        nanoseconds: 538_123_456,
    };
    assert_eq!(
        r#"{"ts":1.538}"#,
        serde_json::to_string(&S { ts }).unwrap(),
        "Higher precision is ignored"
    );
    let ts = Timestamp {
        seconds: -1,
        nanoseconds: 500_000_000,
    };
    assert_eq!(
        r#"{"ts":-1.5}"#,
        serde_json::to_string(&S { ts }).unwrap(),
        "Negative values"
    );
}
