mod utils;

use crate::utils::is_equal;
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::time::{Duration, SystemTime};

/// This is a small non-comprehensive test
#[test]
fn test_duration_systemtime_smoketest() {
    let one_second = Duration::new(1, 0);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsInt(#[serde_as(as = "DurationSeconds")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Secondsf64(#[serde_as(as = "DurationSeconds<f64>")] Duration);

    is_equal(SecondsInt(one_second), expect![[r#"1"#]]);
    is_equal(Secondsf64(one_second), expect![[r#"1.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsInt(#[serde_as(as = "DurationMilliSeconds")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsf64(#[serde_as(as = "DurationMilliSeconds<f64>")] Duration);

    is_equal(MilliSecondsInt(one_second), expect![[r#"1000"#]]);
    is_equal(MilliSecondsf64(one_second), expect![[r#"1000.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsInt(#[serde_as(as = "DurationMicroSeconds")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsf64(#[serde_as(as = "DurationMicroSeconds<f64>")] Duration);

    is_equal(MicroSecondsInt(one_second), expect![[r#"1000000"#]]);
    is_equal(MicroSecondsf64(one_second), expect![[r#"1000000.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsInt(#[serde_as(as = "DurationNanoSeconds")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsf64(#[serde_as(as = "DurationNanoSeconds<f64>")] Duration);

    is_equal(NanoSecondsInt(one_second), expect![[r#"1000000000"#]]);
    is_equal(NanoSecondsf64(one_second), expect![[r#"1000000000.0"#]]);
}

/// This is a small non-comprehensive test
#[test]
fn test_duration_with_frac_systemtime_smoketest() {
    let one_second = Duration::new(1, 0);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsWithFracInt(#[serde_as(as = "DurationSecondsWithFrac")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsWithFracString(#[serde_as(as = "DurationSecondsWithFrac<String>")] Duration);

    is_equal(SecondsWithFracInt(one_second), expect![[r#"1.0"#]]);
    is_equal(SecondsWithFracString(one_second), expect![[r#""1""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsWithFracInt(#[serde_as(as = "DurationMilliSecondsWithFrac")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsWithFracString(
        #[serde_as(as = "DurationMilliSecondsWithFrac<String>")] Duration,
    );

    is_equal(MilliSecondsWithFracInt(one_second), expect![[r#"1000.0"#]]);
    is_equal(
        MilliSecondsWithFracString(one_second),
        expect![[r#""1000""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsWithFracInt(#[serde_as(as = "DurationMicroSecondsWithFrac")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsWithFracString(
        #[serde_as(as = "DurationMicroSecondsWithFrac<String>")] Duration,
    );

    is_equal(
        MicroSecondsWithFracInt(one_second),
        expect![[r#"1000000.0"#]],
    );
    is_equal(
        MicroSecondsWithFracString(one_second),
        expect![[r#""1000000""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsWithFracInt(#[serde_as(as = "DurationNanoSecondsWithFrac")] Duration);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsWithFracString(
        #[serde_as(as = "DurationNanoSecondsWithFrac<String>")] Duration,
    );

    is_equal(
        NanoSecondsWithFracInt(one_second),
        expect![[r#"1000000000.0"#]],
    );
    is_equal(
        NanoSecondsWithFracString(one_second),
        expect![[r#""1000000000""#]],
    );
}

/// This is a small non-comprehensive test
#[test]
fn test_timestamp_systemtime_smoketest() {
    let one_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(1, 0))
        .unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsInt(#[serde_as(as = "TimestampSeconds")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Secondsf64(#[serde_as(as = "TimestampSeconds<f64>")] SystemTime);

    is_equal(SecondsInt(one_second), expect![[r#"1"#]]);
    is_equal(Secondsf64(one_second), expect![[r#"1.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsInt(#[serde_as(as = "TimestampMilliSeconds")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsf64(#[serde_as(as = "TimestampMilliSeconds<f64>")] SystemTime);

    is_equal(MilliSecondsInt(one_second), expect![[r#"1000"#]]);
    is_equal(MilliSecondsf64(one_second), expect![[r#"1000.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsInt(#[serde_as(as = "TimestampMicroSeconds")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsf64(#[serde_as(as = "TimestampMicroSeconds<f64>")] SystemTime);

    is_equal(MicroSecondsInt(one_second), expect![[r#"1000000"#]]);
    is_equal(MicroSecondsf64(one_second), expect![[r#"1000000.0"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsInt(#[serde_as(as = "TimestampNanoSeconds")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsf64(#[serde_as(as = "TimestampNanoSeconds<f64>")] SystemTime);

    is_equal(NanoSecondsInt(one_second), expect![[r#"1000000000"#]]);
    is_equal(NanoSecondsf64(one_second), expect![[r#"1000000000.0"#]]);
}

/// This is a small non-comprehensive test
#[test]
fn test_timestamp_with_frac_systemtime_smoketest() {
    let one_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(1, 0))
        .unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsWithFracInt(#[serde_as(as = "TimestampSecondsWithFrac")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SecondsWithFracString(#[serde_as(as = "TimestampSecondsWithFrac<String>")] SystemTime);

    is_equal(SecondsWithFracInt(one_second), expect![[r#"1.0"#]]);
    is_equal(SecondsWithFracString(one_second), expect![[r#""1""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsWithFracInt(#[serde_as(as = "TimestampMilliSecondsWithFrac")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MilliSecondsWithFracString(
        #[serde_as(as = "TimestampMilliSecondsWithFrac<String>")] SystemTime,
    );

    is_equal(MilliSecondsWithFracInt(one_second), expect![[r#"1000.0"#]]);
    is_equal(
        MilliSecondsWithFracString(one_second),
        expect![[r#""1000""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsWithFracInt(#[serde_as(as = "TimestampMicroSecondsWithFrac")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MicroSecondsWithFracString(
        #[serde_as(as = "TimestampMicroSecondsWithFrac<String>")] SystemTime,
    );

    is_equal(
        MicroSecondsWithFracInt(one_second),
        expect![[r#"1000000.0"#]],
    );
    is_equal(
        MicroSecondsWithFracString(one_second),
        expect![[r#""1000000""#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsWithFracInt(#[serde_as(as = "TimestampNanoSecondsWithFrac")] SystemTime);
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NanoSecondsWithFracString(
        #[serde_as(as = "TimestampNanoSecondsWithFrac<String>")] SystemTime,
    );

    is_equal(
        NanoSecondsWithFracInt(one_second),
        expect![[r#"1000000000.0"#]],
    );
    is_equal(
        NanoSecondsWithFracString(one_second),
        expect![[r#""1000000000""#]],
    );
}
