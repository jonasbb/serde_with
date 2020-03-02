#![cfg(feature = "hex")]

mod utils;

use crate::utils::is_equal;
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, As};

#[test]
fn hex_vec() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeBytes {
        #[serde(with = "As::<Vec<Hex>>")]
        bytes: Vec<Vec<u8>>,
    }
    is_equal(
        SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]],
        },
        r#"{"bytes":["0001020d","0e050607"]}"#,
    );
}
