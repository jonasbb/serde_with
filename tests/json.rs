#![cfg(feature = "json")]

mod utils;

use crate::utils::is_equal;
use serde::{Deserialize, Serialize};
use serde_with::{json::JsonString, serde_as, DisplayFromStr};

#[test]
fn test_nested_json() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString")]
        value: Nested,
    };

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    }

    is_equal(
        Struct {
            value: Nested { value: 444 },
        },
        r#"{"value":"{\"value\":\"444\"}"}"#,
    );
}
