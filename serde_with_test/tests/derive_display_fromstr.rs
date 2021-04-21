//! Test that the derive macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::s_with::{DeserializeFromStr, SerializeDisplay};

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct A;

impl ::std::str::FromStr for A {
    type Err = ::std::string::String;
    fn from_str(_: &str) -> ::std::result::Result<Self, Self::Err> {
        ::std::unimplemented!()
    }
}

impl ::std::fmt::Display for A {
    fn fmt(&self, _: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::unimplemented!()
    }
}
