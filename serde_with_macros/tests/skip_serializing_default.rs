//! Test Cases

use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::skip_serializing_default;

extern crate self as serde_with;

#[allow(missing_docs)]
pub mod __private__ {
    pub fn is_default<T>(value: &T) -> bool
    where
        T: Default + PartialEq,
    {
        value == &T::default()
    }
}

fn never<T>(_t: &T) -> bool {
    false
}

fn custom_default() -> u64 {
    42
}

fn custom_no_default() -> NoDefault {
    NoDefault(1)
}

#[derive(Debug, Eq, PartialEq, Serialize)]
struct NoDefault(u64);

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataBasic {
    a: String,
    b: u64,
    c: bool,
    d: Option<String>,
}

#[test]
fn test_basic() {
    let expected = json!({});
    let data = DataBasic {
        a: String::new(),
        b: 0,
        c: false,
        d: None,
    };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);

    let expected = json!({
        "a": "test",
        "b": 1,
        "c": true,
        "d": "value",
    });
    let data = DataBasic {
        a: "test".to_string(),
        b: 1,
        c: true,
        d: Some("value".to_string()),
    };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataExistingAnnotation {
    #[serde(skip_serializing_if = "String::is_empty")]
    a: String,
    #[serde(skip_serializing_if = "never")]
    #[serde(rename = "name")]
    b: u64,
    c: bool,
}

#[test]
fn test_existing_annotation() {
    let expected = json!({ "name": 0 });
    let data = DataExistingAnnotation {
        a: String::new(),
        b: 0,
        c: false,
    };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DataSerdeDefaultFunction {
    #[serde(default = "custom_default")]
    a: u64,
}

#[test]
fn test_serde_default_function() {
    let expected = json!({});
    let data = DataSerdeDefaultFunction { a: 42 };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
    assert_eq!(data, serde_json::from_value(res).unwrap());

    let expected = json!({ "a": 0 });
    let data = DataSerdeDefaultFunction { a: 0 };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataSerdeDefaultFunctionNoDefault {
    #[serde(default = "custom_no_default")]
    a: NoDefault,
}

#[test]
fn test_serde_default_function_does_not_require_default() {
    let expected = json!({});
    let data = DataSerdeDefaultFunctionNoDefault { a: NoDefault(1) };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);

    let expected = json!({ "a": 2 });
    let data = DataSerdeDefaultFunctionNoDefault { a: NoDefault(2) };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataSerializeAlways {
    #[serialize_always]
    a: String,
    b: u64,
}

#[test]
fn test_serialize_always() {
    let expected = json!({
        "a": "",
    });
    let data = DataSerializeAlways {
        a: String::new(),
        b: 0,
    };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataTuple(String, u64);

#[test]
fn test_tuple() {
    let expected = json!([]);
    let data = DataTuple(String::new(), 0);
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}

#[skip_serializing_default]
#[derive(Debug, Eq, PartialEq, Serialize)]
enum DataEnum {
    Tuple(String, u64),
    Struct { a: String, b: u64 },
}

#[test]
fn test_enum() {
    let expected = json!({
        "Tuple": []
    });
    let data = DataEnum::Tuple(String::new(), 0);
    let res = serde_json::to_value(data).unwrap();
    assert_eq!(expected, res);

    let expected = json!({
        "Struct": {}
    });
    let data = DataEnum::Struct {
        a: String::new(),
        b: 0,
    };
    let res = serde_json::to_value(data).unwrap();
    assert_eq!(expected, res);
}
