//! Test that the with_prefix macros properly name all the types and traits used

// Ensure no prelude is available
#![no_implicit_prelude]
#![allow(dead_code)]

use ::s_with::with_prefix;

with_prefix!(prefix_foobar "foobar_");
