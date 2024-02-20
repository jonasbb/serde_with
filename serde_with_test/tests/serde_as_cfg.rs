//! Test that cfg_eval helps in cfg-gating serde_with attributes

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

#[cfg_attr(test, ::cfg_eval::cfg_eval, ::s_with::serde_as(crate = "::s_with"))]
#[cfg_attr(test, derive(::s::Serialize, ::s::Deserialize))]
#[derive(Debug)]
#[serde(crate = "::s")]
struct S {
    #[cfg_attr(test, serde_as(as = "::s_with::DisplayFromStr"))]
    int: i32,
}

#[test]
fn serde_as_cfg_gated() {
    let s = S { int: 42 };
    // Check serialization
    let s = ::s_json::to_string(&s).unwrap();
    ::std::assert_eq!(s, r#"{"int":"42"}"#);
    let s: S = ::s_json::from_str(&s).unwrap();
    ::std::assert_eq!(s.int, 42);
}
