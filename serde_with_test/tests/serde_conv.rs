//! Test that the serde_conv macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::s_with::serde_conv;
use ::std::borrow::ToOwned;
use ::std::convert::Infallible;
use ::std::num::ParseIntError;
use ::std::result::Result;
use ::std::result::Result::Ok;
use ::std::string::{String, ToString};

serde_conv!(
    BoolAsString,
    bool,
    |x: &bool| ToString::to_string(x),
    |x: String| x.parse()
);

#[derive(::s::Serialize, ::s::Deserialize)]
#[serde(crate = "::s")]
struct S(#[serde(with = "BoolAsString")] bool);

// Also test that lowercase identifier do not cause warnings.
// This makes it look more like a module.
serde_conv!(number, u32, u32_to_string, string_to_u32);

fn u32_to_string(x: &u32) -> String {
    ToString::to_string(x)
}

fn string_to_u32(s: String) -> Result<u32, ParseIntError> {
    s.parse()
}

#[derive(::s::Serialize, ::s::Deserialize)]
#[serde(crate = "::s")]
struct S2(#[serde(with = "number")] u32);

// Test for clippy warning clippy::ptr_arg
// warning: writing `&String` instead of `&str` involves a new object where a slice will do
// https://github.com/jonasbb/serde_with/pull/320
// https://github.com/jonasbb/serde_with/pull/729
serde_conv!(
    pub StringAsHtml,
    String,
    |string: &str| ToOwned::to_owned(string),
    |string: String| -> Result<_, Infallible> {
Ok(string)
    }
);

// Test the support of attributes
serde_conv!(
    #[doc = "Convert number to string"]
    attr1,
    u32,
    u32_to_string,
    string_to_u32
);
serde_conv!(
    #[doc = "Convert number to string"]
    #[allow(dead_code)]
    attr2,
    u32,
    u32_to_string,
    string_to_u32
);
serde_conv!(#[doc = "Convert number to string"] pub attr3, u32, u32_to_string, string_to_u32);
