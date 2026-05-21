//! Test that cfg_eval helps in cfg-gating serde_with attributes

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::cfg_eval::cfg_eval;
use ::s::{Deserialize, Serialize};
use ::s_json::{from_str, to_string};
use ::s_with::serde_as;
use ::std::assert_eq;

#[cfg_attr(test, cfg_eval, serde_as(crate = "::s_with"))]
#[cfg_attr(test, derive(Serialize, Deserialize))]
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
    let s = to_string(&s).unwrap();
    assert_eq!(s, r#"{"int":"42"}"#);
    let s: S = from_str(&s).unwrap();
    assert_eq!(s.int, 42);
}
