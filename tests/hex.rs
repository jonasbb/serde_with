mod utils;

use crate::utils::{check_deserialization, check_error_deserialization, is_equal};
use serde::{Deserialize, Serialize};
use serde_with::{
    formats::{Lowercase, Uppercase},
    hex::Hex,
    serde_as,
};

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

    // Check mixed case deserialization
    check_deserialization(
        SomeBytes {
            bytes: vec![vec![0xaa, 0xbc, 0xff], vec![0xe0, 0x7d]],
        },
        r#"{"bytes":["aaBCff","E07d"]}"#,
    );

    check_error_deserialization::<SomeBytes>(
        r#"{"bytes":["0"]}"#,
        "Odd number of digits at line 1 column 14",
    );
    check_error_deserialization::<SomeBytes>(
        r#"{"bytes":["zz"]}"#,
        "Invalid character \'z\' at position 0 at line 1 column 15",
    );
}

#[test]
fn hex_vec_lowercase() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeBytes {
        #[serde_as(as = "Vec<Hex<Lowercase>>")]
        bytes: Vec<Vec<u8>>,
    }
    is_equal(
        SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]],
        },
        r#"{"bytes":["0001020d","0e050607"]}"#,
    );

    // Check mixed case deserialization
    check_deserialization(
        SomeBytes {
            bytes: vec![vec![0xaa, 0xbc, 0xff], vec![0xe0, 0x7d]],
        },
        r#"{"bytes":["aaBCff","E07d"]}"#,
    );
}

#[test]
fn hex_vec_uppercase() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct SomeBytes {
        #[serde_as(as = "Vec<Hex<Uppercase>>")]
        bytes: Vec<Vec<u8>>,
    }
    is_equal(
        SomeBytes {
            bytes: vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]],
        },
        r#"{"bytes":["0001020D","0E050607"]}"#,
    );

    // Check mixed case deserialization
    check_deserialization(
        SomeBytes {
            bytes: vec![vec![0xaa, 0xbc, 0xff], vec![0xe0, 0x7d]],
        },
        r#"{"bytes":["aaBCff","E07d"]}"#,
    );
}
