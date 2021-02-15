//! Test that the `serde_as(crate = "...")` argument allows compilation when the crate isn't
//! available at path `::serde_with`

#![allow(dead_code)]

use s::{Deserialize, Serialize};
use s_with::serde_as;

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

#[serde_as(crate = "s_with")]
#[derive(Deserialize, Serialize)]
#[serde(crate = "s")]
struct Data {
    #[serde_as(as = "_")]
    a: u32,
}
