//! Test that the `serde_as(crate = "...")` argument allows compilation when the crate isn't
//! available at path `::serde_with`

use s::{Deserialize, Serialize};
use s_with::serde_as;

#[serde(crate = "s")]
#[serde_as(crate = "s_with")]
#[derive(Deserialize, Serialize)]
struct Data {
    #[serde_as(as = "_")]
    a: u32,
}
