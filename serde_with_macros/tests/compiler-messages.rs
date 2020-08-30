// This test fails for older compiler versions since the error messages are different.
#[rustversion::attr(before(1.43), ignore)]
#[cfg_attr(tarpaulin, ignore)]
// The error messages are more detailed on nightly, thus breaking the test.
#[rustversion::attr(nightly, ignore)]
#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
