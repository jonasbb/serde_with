use ::schemars_0_8::JsonSchema;
use serde_with::*;

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
