//! Test that the derive macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code, unused_imports)]

use ::s_with::{DeserializeFromStr, SerializeDisplay};
// Needed for 1.46, unused in 1.50
use ::std::panic;

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

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct G<T>(T);

impl<T> ::std::str::FromStr for G<T> {
    type Err = ::std::string::String;
    fn from_str(_: &str) -> ::std::result::Result<Self, Self::Err> {
        ::std::unimplemented!()
    }
}

impl<T> ::std::fmt::Display for G<T> {
    fn fmt(&self, _: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::unimplemented!()
    }
}

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct LT<'a>(&'a ());

impl ::std::str::FromStr for LT<'_> {
    type Err = ::std::string::String;
    fn from_str(_: &str) -> ::std::result::Result<Self, Self::Err> {
        ::std::unimplemented!()
    }
}

impl ::std::fmt::Display for LT<'_> {
    fn fmt(&self, _: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::unimplemented!()
    }
}
