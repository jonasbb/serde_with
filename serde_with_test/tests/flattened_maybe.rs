//! Test that flattened_maybe properly names all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

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
    ::s_json::from_str::<S>(j).unwrap();

    // and non-flattened versions.
    let j = r#" {"t":{"i":1}} "#;
    ::s_json::from_str::<S>(j).unwrap();

    // Ensure that the value is given
    let j = r#" {} "#;
    ::s_json::from_str::<S>(j).unwrap_err();

    // and only occurs once, not multiple times.
    let j = r#" {"i":1,"t":{"i":1}} "#;
    ::s_json::from_str::<S>(j).unwrap_err();
}
