extern crate pretty_assertions;
extern crate serde;
extern crate serde_json;
extern crate serde_with_macros;

use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::skip_serializing_null;

macro_rules! test {
    ($fn:ident, $struct:ident) => {
        #[test]
        fn $fn() {
            let expected = json!({});
            let data = $struct {
                a: None,
                b: None,
                c: None,
                d: None,
            };
            let res = serde_json::to_value(&data).unwrap();
            assert_eq!(expected, res);
            assert_eq!(data, serde_json::from_value(res).unwrap());
        }
    };
}

macro_rules! test_tuple {
    ($fn:ident, $struct:ident) => {
        #[test]
        fn $fn() {
            let expected = json!([]);
            let data = $struct(None, None);
            let res = serde_json::to_value(&data).unwrap();
            assert_eq!(expected, res);
        }
    };
}

#[skip_serializing_null]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DataBasic {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: Option<String>,
}
test!(test_basic, DataBasic);

#[skip_serializing_null]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DataFullyQualified {
    a: ::std::option::Option<String>,
    b: std::option::Option<String>,
    c: ::std::option::Option<i64>,
    d: core::option::Option<String>,
}
test!(test_fully_qualified, DataFullyQualified);

#[skip_serializing_null]
#[derive(Debug, Eq, PartialEq, Serialize)]
struct DataTuple(Option<String>, std::option::Option<String>);
test_tuple!(test_tuple, DataTuple);

#[skip_serializing_null]
#[derive(Debug, Eq, PartialEq, Serialize)]
enum DataEnum {
    Tuple(Option<i64>, std::option::Option<bool>),
    Struct {
        a: Option<String>,
        b: Option<String>,
    },
}

#[test]
fn test_enum() {
    let expected = json!({
        "Tuple": []
    });
    let data = DataEnum::Tuple(None, None);
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);

    let expected = json!({
        "Struct": {}
    });
    let data = DataEnum::Struct { a: None, b: None };
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
}
