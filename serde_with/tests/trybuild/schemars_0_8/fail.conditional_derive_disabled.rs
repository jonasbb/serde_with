use schemars_0_8::JsonSchema;
use serde::Serialize;

extern crate schemars_0_8 as schemars;

#[serde_with::serde_as]
#[derive(Serialize)]
#[cfg_attr(any(), derive(JsonSchema))]
struct Conditional {
    #[serde_as(as = "_")]
    field: u32
}

fn assert_implements_jsonschema<T: JsonSchema>() {}

fn main() {
    assert_implements_jsonschema::<Conditional>();
}
