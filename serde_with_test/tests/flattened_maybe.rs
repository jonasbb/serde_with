//! Test that flattened_maybe properly names all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code, unused_imports)]

use ::s_with as serde_with;
// Needed for 1.46, unused in 1.50
use ::std::panic;

// The macro creates custom deserialization code.
// You need to specify a function name and the field name of the flattened field.
::s_with::flattened_maybe!(deserialize_t, "t");
// Setup the types
#[derive(::s::Deserialize, Debug)]
#[serde(crate = "::s")]
struct S {
    #[serde(flatten, deserialize_with = "deserialize_t")]
    t: T,
}

#[derive(::s::Deserialize, Debug)]
#[serde(crate = "::s")]
struct T {
    i: i32,
}

#[test]
fn flattened_maybe() {
    // Supports both flattened
    let j = r#" {"i":1} "#;
    ::std::assert!(::s_json::from_str::<S>(j).is_ok());

    // and non-flattened versions.
    let j = r#" {"t":{"i":1}} "#;
    ::std::assert!(::s_json::from_str::<S>(j).is_ok());

    // Ensure that the value is given
    let j = r#" {} "#;
    ::std::assert!(::s_json::from_str::<S>(j).is_err());

    // and only occurs once, not multiple times.
    let j = r#" {"i":1,"t":{"i":1}} "#;
    ::std::assert!(::s_json::from_str::<S>(j).is_err());
}
