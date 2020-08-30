#![allow(dead_code)]

use expect_test::Expect;
use pretty_assertions::assert_eq;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[rustversion::attr(since(1.46), track_caller)]
pub fn is_equal<T>(value: T, s: &str)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    assert_eq!(
        serde_json::from_str::<T>(s).unwrap(),
        value,
        "Deserialization differs from expected value."
    );
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        s,
        "Serialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn is_equal_expect<T>(value: T, expected: Expect)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    let serialized = serde_json::to_string_pretty(&value).unwrap();
    expected.assert_eq(&serialized);
    assert_eq!(
        serde_json::from_str::<T>(&serialized).unwrap(),
        value,
        "Deserialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_deserialization<T>(value: T, deserialize_from: &str)
where
    T: Debug + DeserializeOwned + PartialEq,
{
    assert_eq!(
        serde_json::from_str::<T>(deserialize_from).unwrap(),
        value,
        "Deserialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_serialization<T>(value: T, serialize_to: &str)
where
    T: Debug + PartialEq + Serialize,
{
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        serialize_to,
        "Serialization differs from expected value."
    );
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_serialization_expect<T>(value: T, serialize_to: Expect)
where
    T: Debug + PartialEq + Serialize,
{
    serialize_to.assert_eq(&serde_json::to_string_pretty(&value).unwrap());
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_error_deserialization<T>(deserialize_from: &str, error_msg: &str)
where
    T: Debug + DeserializeOwned + PartialEq,
{
    assert_eq!(
        serde_json::from_str::<T>(deserialize_from)
            .unwrap_err()
            .to_string(),
        error_msg
    )
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn check_error_deserialization_expect<T>(deserialize_from: &str, error_msg: Expect)
where
    T: Debug + DeserializeOwned + PartialEq,
{
    error_msg.assert_eq(
        &serde_json::from_str::<T>(deserialize_from)
            .unwrap_err()
            .to_string(),
    )
}
