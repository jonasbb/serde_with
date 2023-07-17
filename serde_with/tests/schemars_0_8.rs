#![allow(
  // clippy is broken and shows wrong warnings
  // clippy on stable does not know yet about the lint name
  unknown_lints,
  // https://github.com/rust-lang/rust-clippy/issues/8867
  clippy::derive_partial_eq_without_eq,
)]

extern crate alloc;

use expect_test::expect;
use schemars_0_8::JsonSchema;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString};

#[track_caller]
fn verify_valid_json_schema<T: JsonSchema + serde::Serialize>(value: T) {
    let schema = ::schemars_0_8::schema_for!(T);
    let compiled = jsonschema::JSONSchema::compile(&serde_json::to_value(&schema).unwrap())
        .expect("Invalid schema");
    let instance = serde_json::to_value(&value).unwrap();

    assert!(compiled.is_valid(&instance));
}

#[test]
fn schemars_basic() {
    use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};

    #[serde_with::serde_as(schemars = "true")]
    #[derive(JsonSchema, Serialize)]
    #[schemars(crate = "::schemars_0_8")]
    struct A {
        a: u32,
        #[serde_as(as = "DisplayFromStr")]
        b: u32,
        #[serde_as(as = "Box<DisplayFromStr>")]
        c: Box<u32>,
        #[serde_as(as = "Box<_>")]
        d: Box<u32>,
        #[serde_as(as = "Vec<DisplayFromStr>")]
        e: Vec<u32>,
        #[serde_as(as = "NoneAsEmptyString")]
        f: Option<String>,
    }

    let schema = ::schemars_0_8::schema_for!(A);
    expect![[r##"
        {
          "$schema": "http://json-schema.org/draft-07/schema#",
          "title": "A",
          "type": "object",
          "required": [
            "a",
            "b",
            "c",
            "d",
            "e",
            "f"
          ],
          "properties": {
            "a": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0.0
            },
            "b": {
              "$ref": "#/definitions/String"
            },
            "c": {
              "$ref": "#/definitions/String"
            },
            "d": {
              "$ref": "#/definitions/uint32"
            },
            "e": {
              "$ref": "#/definitions/Array_of_String"
            },
            "f": {
              "$ref": "#/definitions/Nullable_String"
            }
          },
          "definitions": {
            "Array_of_String": {
              "type": "array",
              "items": {
                "$ref": "#/definitions/String"
              }
            },
            "Nullable_String": {
              "type": [
                "string",
                "null"
              ]
            },
            "String": {
              "type": "string"
            },
            "uint32": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0.0
            }
          }
        }"##]]
    .assert_eq(&serde_json::to_string_pretty(&schema).unwrap());

    verify_valid_json_schema(A {
        a: 0,
        b: 1,
        c: Box::new(2),
        d: Box::new(3),
        e: vec![4, 5],
        f: None,
    });
}
