#![allow(dead_code)]

use expect_test::Expect;
use pretty_assertions::assert_eq;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[rustversion::attr(since(1.46), track_caller)]
pub fn is_equal<T>(value: T, expected: Expect)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    let serialized = serde_json::to_string_pretty(&value).unwrap();
    expected.assert_eq(&serialized);
    assert_eq!(
        value,
        serde_json::from_str::<T>(&serialized).unwrap(),
        "Deserialization differs from expected value."
    );
}

/// Like [`is_equal`] but not pretty-print
#[rustversion::attr(since(1.46), track_caller)]
pub fn is_equal_compact<T>(value: T, expected: Expect)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    let serialized = serde_json::to_string(&value).unwrap();
    expected.assert_eq(&serialized);
    assert_eq!(
        value,
        serde_json::from_str::<T>(&serialized).unwrap(),
        "Deserialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_deserialization<T>(value: T, deserialize_from: &str)
where
    T: Debug + DeserializeOwned + PartialEq,
{
    assert_eq!(
        value,
        serde_json::from_str::<T>(deserialize_from).unwrap(),
        "Deserialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_serialization<T>(value: T, serialize_to: Expect)
where
    T: Debug + Serialize,
{
    serialize_to.assert_eq(&serde_json::to_string_pretty(&value).unwrap());
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_error_deserialization<T>(deserialize_from: &str, error_msg: Expect)
where
    T: Debug + DeserializeOwned,
{
    error_msg.assert_eq(
        &serde_json::from_str::<T>(deserialize_from)
            .unwrap_err()
            .to_string(),
    )
}
