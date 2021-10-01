mod utils;

use crate::utils::{check_deserialization, check_error_deserialization, is_equal};
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, Standard, UrlSafe};
use serde_with::formats::{Padded, Unpadded};
use serde_with::serde_as;

#[test]
fn base64_vec() {
    let check_equal = vec![vec![0, 1, 2, 13], vec![14, 5, 6, 7]];
    let check_deser = vec![vec![0xaa, 0xbc, 0xff], vec![0xe0, 0x7d], vec![0xe0, 0x7d]];
    let check_deser_from = r#"["qrz/","4H0=","4H0"]"#;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct BDefault(#[serde_as(as = "Vec<Base64>")] Vec<Vec<u8>>);

    is_equal(
        BDefault(check_equal.clone()),
        expect![[r#"
            [
              "AAECDQ==",
              "DgUGBw=="
            ]"#]],
    );

    // Check mixed padding deserialization
    check_deserialization(BDefault(check_deser.clone()), check_deser_from);

    check_error_deserialization::<BDefault>(
        r#"["0"]"#,
        expect![[r#"Encoded text cannot have a 6-bit remainder. at line 1 column 5"#]],
    );
    check_error_deserialization::<BDefault>(
        r#"["zz"]"#,
        expect![[r#"Invalid last symbol 122, offset 1. at line 1 column 6"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct BPadded(#[serde_as(as = "Vec<Base64<Standard, Padded>>")] Vec<Vec<u8>>);

    is_equal(
        BPadded(check_equal.clone()),
        expect![[r#"
            [
              "AAECDQ==",
              "DgUGBw=="
            ]"#]],
    );
    check_deserialization(BPadded(check_deser.clone()), check_deser_from);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct BUnpadded(#[serde_as(as = "Vec<Base64<Standard, Unpadded>>")] Vec<Vec<u8>>);

    is_equal(
        BUnpadded(check_equal.clone()),
        expect![[r#"
            [
              "AAECDQ",
              "DgUGBw"
            ]"#]],
    );
    check_deserialization(BUnpadded(check_deser.clone()), check_deser_from);
}
