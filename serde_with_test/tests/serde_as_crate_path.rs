//! Test that the `serde_as(crate = "...")` argument allows compilation when the crate isn't
//! available at path `::serde_with`

#![no_implicit_prelude]
#![allow(dead_code)]

use ::s::{Deserialize, Serialize};
use ::s_with::serde_as;

#[serde_as(crate = "::s_with")]
#[derive(Deserialize, Serialize)]
#[serde(crate = "::s")]
struct Data {
    #[serde_as(as = "_")]
    a: u32,
}
