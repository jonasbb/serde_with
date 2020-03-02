#![cfg(feature = "json")]

mod utils;

use crate::utils::is_equal;
use serde::{Deserialize, Serialize};
use serde_with::{json::JsonString, As, DisplayFromStr};

#[test]
fn test_nested_json() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<JsonString>")]
        value: Nested,
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        #[serde(with = "As::<DisplayFromStr>")]
        value: u32,
    }

    is_equal(
        Struct {
            value: Nested { value: 444 },
        },
        r#"{"value":"{\"value\":\"444\"}"}"#,
    );
}
