#![allow(dead_code)]

use s_with as serde_with;

// Ensure that types from the environment do not infect the macro
#[allow(unused_imports)]
use crate::Option::*;
mod std {}
type Result = ();
enum Option {
    Some,
    None(()),
    Ok,
    Err,
}

// The macro creates custom deserialization code.
// You need to specify a function name and the field name of the flattened field.
s_with::flattened_maybe!(deserialize_t, "t");
// Setup the types
#[derive(s::Deserialize, Debug)]
#[serde(crate = "s")]
struct S {
    #[serde(flatten, deserialize_with = "deserialize_t")]
    t: T,
}

#[derive(s::Deserialize, Debug)]
#[serde(crate = "s")]
struct T {
    i: i32,
}

#[test]
fn flattened_maybe() {
    // Supports both flattened
    let j = r#" {"i":1} "#;
    assert!(s_json::from_str::<S>(j).is_ok());

    // and non-flattened versions.
    let j = r#" {"t":{"i":1}} "#;
    assert!(s_json::from_str::<S>(j).is_ok());

    // Ensure that the value is given
    let j = r#" {} "#;
    assert!(s_json::from_str::<S>(j).is_err());

    // and only occurs once, not multiple times.
    let j = r#" {"i":1,"t":{"i":1}} "#;
    assert!(s_json::from_str::<S>(j).is_err());
}
