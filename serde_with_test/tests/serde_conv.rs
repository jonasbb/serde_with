//! Test that the serde_conv macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::s_with::serde_conv;

serde_conv!(
    BoolAsString,
    bool,
    |x: &bool| ::std::string::ToString::to_string(x),
    |x: ::std::string::String| x.parse()
);

#[derive(::s::Serialize, ::s::Deserialize)]
#[serde(crate = "::s")]
struct S(#[serde(with = "BoolAsString")] bool);

// Also test that lowercase identifier do not cause warnings.
// This makes it look more like a module.
serde_conv!(number, u32, u32_to_string, string_to_u32);

fn u32_to_string(x: &u32) -> ::std::string::String {
    ::std::string::ToString::to_string(x)
}

fn string_to_u32(
    s: ::std::string::String,
) -> ::std::result::Result<u32, ::std::num::ParseIntError> {
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
    ::std::string::String,
    |string: &str| ::std::borrow::ToOwned::to_owned(string),
    |string: ::std::string::String| -> ::std::result::Result<_, ::std::convert::Infallible> {
::std::result::Result::Ok(string)
    }
);
