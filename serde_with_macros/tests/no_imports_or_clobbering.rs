use serde_with_macros::{DeserializeFromStr, SerializeDisplay};

// We check that the macros result in valid code even in
// absence of a FromStr import and with a clobbered Result type

#[allow(dead_code)]
type Result = ();

#[derive(DeserializeFromStr, SerializeDisplay)]
struct A;

impl std::str::FromStr for A {
    type Err = String;
    fn from_str(_: &str) -> std::result::Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl std::fmt::Display for A {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
