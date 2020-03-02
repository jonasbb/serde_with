#![cfg(feature = "hex")]

use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, As};

#[test]
fn hex_vec() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeBytes {
        #[serde(with = "As::<Vec<Hex>>")]
        bytes: Vec<Vec<u8>>,
    }

    assert_eq!(
        serde_json::to_string(&SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]]
        })
        .unwrap(),
        "{\"bytes\":[\"0001020d\",\"0e050607\"]}"
    );

    assert_eq!(
        SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]]
        },
        serde_json::from_str("{\"bytes\":[\"0001020d\",\"0e050607\"]}").unwrap(),
    );
}
