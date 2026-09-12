//! Test Cases

mod utils;

use crate::utils::is_equal;
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::{json::JsonString, serde_as, DisplayFromStr};
use std::collections::BTreeMap;

#[test]
fn test_jsonstring() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString")]
        value: Nested,
    }

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
        expect![[r#"
            {
              "value": "{\"value\":\"444\"}"
            }"#]],
    );
}

#[test]
fn test_jsonstring_nested() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString<Vec<(JsonString, _)>>")]
        value: BTreeMap<[u8; 2], u32>,
    }

    is_equal(
        Struct {
            value: BTreeMap::from([([1, 2], 3), ([4, 5], 6)]),
        },
        expect![[r#"
            {
              "value": "[[\"[1,2]\",3],[\"[4,5]\",6]]"
            }"#]],
    );
}

/// Nested JSON in an adjacently tagged enum, i.e., the original use case of
/// <https://github.com/jonasbb/serde_with/issues/499>.
#[test]
fn test_jsonstring_on_newtype_variant() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(tag = "messageType", content = "content")]
    enum Message {
        Text(String),
        #[serde_as(as = "JsonString")]
        Object(Nested),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        id: u32,
    }

    is_equal(
        Message::Object(Nested { id: 7 }),
        expect![[r#"
            {
              "messageType": "Object",
              "content": "{\"id\":7}"
            }"#]],
    );
}
