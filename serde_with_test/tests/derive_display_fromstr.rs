//! Test that the derive macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::std::fmt::{self, Display};
use ::std::result::Result;
use ::std::str::FromStr;
use ::std::string::String;
use ::std::unimplemented;

use ::s_with::{DeserializeFromStr, SerializeDisplay};

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct A;

impl FromStr for A {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Display for A {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct G<T>(T);

impl<T> FromStr for G<T> {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl<T> Display for G<T> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

// Ensure the common 1-letter generics are not used by the derive macros
// The macros use:
// D: Deserializer
// S: Serializer
// E: Error
// https://github.com/jonasbb/serde_with/pull/526
#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct MoreG<D, E, S>(D, E, S);

impl<D, E, S> FromStr for MoreG<D, E, S> {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl<D, E, S> Display for MoreG<D, E, S> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "::s_with")]
struct LT<'a>(&'a ());

impl FromStr for LT<'_> {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Display for LT<'_> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
