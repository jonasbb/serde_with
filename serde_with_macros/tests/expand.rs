// #[rustversion::attr(not(nightly), ignore)]
#[test]
fn expandtest() {
    macrotest::expand("tests/expand/*.rs");
}
