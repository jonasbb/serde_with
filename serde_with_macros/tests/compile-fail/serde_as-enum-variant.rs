use serde::{Deserialize, Serialize};
use serde_with_macros::serde_as;

/// `serde_as` on a variant is only translated for newtype variants.
/// For every other shape the attribute belongs on the field.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub enum NotANewtypeVariant {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    Unit,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    Tuple(u32, u32),
    #[serde_as(as = "serde_with::DisplayFromStr")]
    Struct { a: u32 },
}

/// Writing `serde_as` in both places is ambiguous.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub enum VariantAndField {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    Value(#[serde_as(as = "serde_with::DisplayFromStr")] u32),
}

fn main() {}
