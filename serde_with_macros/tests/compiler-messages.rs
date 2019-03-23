extern crate rustc_version;
extern crate trybuild;

use rustc_version::{version, Version};

#[test]
fn compile_test() {
    // This test fails for older compiler versions since the error messages are different.
    if version().unwrap() >= Version::parse("1.35.0").unwrap() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile-fail/*.rs");
    }
}
