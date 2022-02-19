//! Test that the derive macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code, unused_imports)]

use ::s_with::{DeserializeAs, SerializeAs};
// Needed for 1.46, unused in 1.50
use ::std::panic;

mod other_crate {
    #[derive(::s::Serialize)]
    #[serde(crate = "::s")]
    pub struct Duration {
        pub secs: i64,
        pub nanos: i32,
    }

    #[derive(::s::Serialize)]
    #[serde(crate = "::s")]
    pub struct Wrapper<'a, T> {
        pub a: &'a str,
        pub b: T,
    }
}

// Test basic usage of the derive macros
#[derive(::s::Serialize, ::s::Deserialize, SerializeAs, DeserializeAs)]
#[serde(crate = "::s", remote = "other_crate::Duration")]
#[serde_with(crate = "::s_with")]
struct DurationDef {
    secs: i64,
    nanos: i32,
}

// More complicated case using lifetimes and generics
#[derive(::s::Serialize, SerializeAs)]
#[serde(crate = "::s", remote = "other_crate::Wrapper")]
#[serde_with(crate = "::s_with")]
struct WrapperDef<'a, T> {
    pub a: &'a str,
    pub b: T,
}
