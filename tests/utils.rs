#![allow(dead_code)]

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

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
