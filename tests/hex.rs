mod utils;

use crate::utils::is_equal;
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};

#[test]
fn hex_vec() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeBytes {
        #[serde_as(as = "Vec<Hex>")]
        bytes: Vec<Vec<u8>>,
    }
    is_equal(
        SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]],
        },
        r#"{"bytes":["0001020d","0e050607"]}"#,
    );
}
