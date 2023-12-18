use crate::utils::{check_matches_schema, check_valid_json_schema};
use ::schemars_0_8::JsonSchema;
use serde::Serialize;
use serde_json::json;
use serde_with::*;

// This avoids us having to add `#[schemars(crate = "::schemars_0_8")]` all
// over the place. We're not testing that and it is inconvenient.
extern crate schemars_0_8 as schemars;

mod utils;

#[test]
fn schemars_basic() {
    use ::schemars_0_8::JsonSchema;
    use serde::Serialize;

    #[serde_with::serde_as]
    #[derive(JsonSchema, Serialize)]
    #[schemars(crate = "::schemars_0_8")]
    struct Basic {
        /// Basic field, no attribute
        bare_field: u32,

        /// Field that directly uses DisplayFromStr
        #[serde_as(as = "DisplayFromStr")]
        display_from_str: u32,

        /// Same does not implement JsonSchema directly so this checks that the
        /// correct schemars attribute was injected.
        #[serde_as(as = "Same")]
        same: u32,

        /// This checks that Same still works when wrapped in a box.
        #[serde_as(as = "Box<Same>")]
        box_same: Box<u32>,

        /// Same thing, but with a Vec this time.
        #[serde_as(as = "Vec<_>")]
        vec_same: Vec<u32>,
    }

    let schema = ::schemars_0_8::schema_for!(Basic);
    let value = serde_json::to_value(schema).expect("schema could not be serialized");

    let expected = serde_json::json!({
      "$schema": "http://json-schema.org/draft-07/schema#",
      "title": "Basic",
      "type": "object",
      "required": [
        "bare_field",
        "box_same",
        "display_from_str",
        "same",
        "vec_same"
      ],
      "properties": {
        "bare_field": {
          "description": "Basic field, no attribute",
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "box_same": {
          "description": "This checks that Same still works when wrapped in a box.",
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "display_from_str": {
          "description": "Field that directly uses DisplayFromStr",
          "type": "string"
        },
        "same": {
          "description": "Same does not implement JsonSchema directly so this checks that the correct schemars attribute was injected.",
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "vec_same": {
          "description": "Same thing, but with a Vec this time.",
          "type": "array",
          "items": {
            "type": "integer",
            "format": "uint32",
            "minimum": 0.0
          }
        }
      }
    });

    assert_eq!(value, expected);
}

mod derive {
    use super::*;

    #[serde_with::serde_as]
    #[derive(Serialize)]
    #[cfg_attr(all(), derive(JsonSchema))]
    struct Enabled {
        #[serde_as(as = "DisplayFromStr")]
        field: u32,
    }

    #[serde_with::serde_as]
    #[derive(Serialize)]
    #[cfg_attr(any(), derive(JsonSchema))]
    struct Disabled {
        // If we are incorrectly adding `#[schemars(with = ...)]` attributes
        // then we should get an error on this field.
        #[serde_as(as = "DisplayFromStr")]
        field: u32,
    }

    #[test]
    fn test_enabled_has_correct_schema() {
        check_valid_json_schema(&Enabled { field: 77 });
    }
}

mod array {
    use super::*;

    #[serde_with::serde_as]
    #[derive(JsonSchema, Serialize)]
    struct FixedArray {
        #[serde_as(as = "[_; 3]")]
        array: [u32; 3],
    }

    #[test]
    fn test_serialized_is_valid() {
        let array = FixedArray { array: [1, 2, 3] };

        check_valid_json_schema(&array);
    }

    #[test]
    fn test_valid_json() {
        let value = json!({ "array": [1, 2, 3] });
        check_matches_schema::<FixedArray>(&value);
    }

    #[test]
    #[should_panic]
    fn test_too_short() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [1],
        }));
    }

    #[test]
    #[should_panic]
    fn test_too_long() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [1, 2, 3, 4]
        }));
    }

    #[test]
    #[should_panic]
    fn test_wrong_item_type() {
        check_matches_schema::<FixedArray>(&json!({
            "array": ["1", "2", "3"]
        }));
    }

    #[test]
    #[should_panic]
    fn test_oob_item() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [-1, 0x1_0000_0000i64, 32]
        }))
    }
}
