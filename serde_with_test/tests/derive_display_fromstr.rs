#![allow(dead_code)]

use s_with::{DeserializeFromStr, SerializeDisplay};

// We check that the macros result in valid code even in
// absence of a FromStr import and with a clobbered Result type
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

#[derive(DeserializeFromStr, SerializeDisplay)]
#[serde_with(crate = "s_with")]
struct A;

impl ::std::str::FromStr for A {
    type Err = String;
    fn from_str(_: &str) -> ::std::result::Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl ::std::fmt::Display for A {
    fn fmt(&self, _: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        unimplemented!()
    }
}
