// This test fails for older compiler versions since the error messages are different.
#[rustversion::attr(before(1.38), ignore)]
// TODO The error messages are more detailed on nightly, thus breaking the test.
#[rustversion::attr(nightly, ignore)]
#[test]
fn compile_test() {
    // This test does not work under tarpaulin, so skip it if detected
    if std::env::var("TARPAULIN") == Ok("1".to_string()) {
        return;
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}
