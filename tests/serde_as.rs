mod utils;

use crate::utils::{
    check_deserialization, check_error_deserialization, check_serialization, is_equal,
};
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::{
    formats::Flexible, serde_as, BytesOrString, DefaultOnError, DefaultOnNull, DisplayFromStr,
    DurationSeconds, DurationSecondsWithFrac, NoneAsEmptyString, Same, TimestampSeconds,
    TimestampSecondsWithFrac,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, LinkedList, VecDeque},
    rc::Rc,
    sync::Arc,
    time::{Duration, SystemTime},
};

#[test]
fn test_box() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "Box<DisplayFromStr>")] Box<u32>);

    is_equal(S(Box::new(123)), expect![[r#""123""#]]);
}

#[test]
fn test_option() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "_")] Option<u32>);

    is_equal(S(None), expect![[r#"null"#]]);
    is_equal(S(Some(9)), expect![[r#"9"#]]);
    check_error_deserialization::<S>(
        r#"{}"#,
        expect![[r#"invalid type: map, expected u32 at line 1 column 0"#]],
    );
    check_error_deserialization::<S>(
        r#"{}"#,
        expect![[r#"invalid type: map, expected u32 at line 1 column 0"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "_")]
        value: Option<u32>,
    };
    check_error_deserialization::<Struct>(
        r#"{}"#,
        expect![[r#"missing field `value` at line 1 column 2"#]],
    );
}

#[test]
fn test_display_fromstr() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "DisplayFromStr")] u32);

    is_equal(S(123), expect![[r#""123""#]]);
}

#[test]
fn test_tuples() {
    use std::net::IpAddr;
    let ip = "1.2.3.4".parse().unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S1(#[serde_as(as = "(DisplayFromStr,)")] (u32,));
    is_equal(
        S1((1,)),
        expect![[r#"
            [
              "1"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2a(#[serde_as(as = "(DisplayFromStr, DisplayFromStr)")] (u32, IpAddr));
    is_equal(
        S2a((555_888, ip)),
        expect![[r#"
            [
              "555888",
              "1.2.3.4"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2b(#[serde_as(as = "(_, DisplayFromStr)")] (u32, IpAddr));
    is_equal(
        S2b((987, ip)),
        expect![[r#"
            [
              987,
              "1.2.3.4"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2c(#[serde_as(as = "(Same, DisplayFromStr)")] (u32, IpAddr));
    is_equal(
        S2c((987, ip)),
        expect![[r#"
            [
              987,
              "1.2.3.4"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S6(
        #[serde_as(as = "(Same, Same, Same, Same, Same, Same)")] (u8, u16, u32, i8, i16, i32),
    );
    is_equal(
        S6((8, 16, 32, -8, 16, -32)),
        expect![[r#"
            [
              8,
              16,
              32,
              -8,
              16,
              -32
            ]"#]],
    );
}

#[test]
fn test_arrays() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S0(#[serde_as(as = "[DisplayFromStr; 0]")] [u32; 0]);
    is_equal(S0([]), expect![[r#"[]"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S1(#[serde_as(as = "[DisplayFromStr; 1]")] [u32; 1]);
    is_equal(
        S1([1]),
        expect![[r#"
            [
              "1"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2(#[serde_as(as = "[Same; 2]")] [u32; 2]);
    is_equal(
        S2([11, 22]),
        expect![[r#"
            [
              11,
              22
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S32(#[serde_as(as = "[Same; 32]")] [u32; 32]);
    is_equal(
        S32([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]),
        expect![[r#"
            [
              0,
              1,
              2,
              3,
              4,
              5,
              6,
              7,
              8,
              9,
              10,
              11,
              12,
              13,
              14,
              15,
              16,
              17,
              18,
              19,
              20,
              21,
              22,
              23,
              24,
              25,
              26,
              27,
              28,
              29,
              30,
              31
            ]"#]],
    );
}

#[test]
fn test_sequence_like_types() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S1(#[serde_as(as = "Box<[Same]>")] Box<[u32]>);
    is_equal(
        S1(vec![1, 2, 3, 99].into()),
        expect![[r#"
            [
              1,
              2,
              3,
              99
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2(#[serde_as(as = "BTreeSet<Same>")] BTreeSet<u32>);
    is_equal(
        S2(vec![1, 2, 3, 99].into_iter().collect()),
        expect![[r#"
            [
              1,
              2,
              3,
              99
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S3(#[serde_as(as = "LinkedList<Same>")] LinkedList<u32>);
    is_equal(
        S3(vec![1, 2, 3, 99].into_iter().collect()),
        expect![[r#"
            [
              1,
              2,
              3,
              99
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S4(#[serde_as(as = "Vec<Same>")] Vec<u32>);
    is_equal(
        S4(vec![1, 2, 3, 99]),
        expect![[r#"
            [
              1,
              2,
              3,
              99
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S5(#[serde_as(as = "VecDeque<Same>")] VecDeque<u32>);
    is_equal(
        S5(vec![1, 2, 3, 99].into()),
        expect![[r#"
            [
              1,
              2,
              3,
              99
            ]"#]],
    );
}

#[test]
fn test_map_as_tuple_list() {
    use std::net::IpAddr;
    let ip = "1.2.3.4".parse().unwrap();
    let ip2 = "255.255.255.255".parse().unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SB(#[serde_as(as = "Vec<(DisplayFromStr, DisplayFromStr)>")] BTreeMap<u32, IpAddr>);

    let map: BTreeMap<_, _> = vec![(1, ip), (10, ip), (200, ip2)].into_iter().collect();
    is_equal(
        SB(map.clone()),
        expect![[r#"
            [
              [
                "1",
                "1.2.3.4"
              ],
              [
                "10",
                "1.2.3.4"
              ],
              [
                "200",
                "255.255.255.255"
              ]
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SB2(#[serde_as(as = "Vec<(Same, DisplayFromStr)>")] BTreeMap<u32, IpAddr>);

    is_equal(
        SB2(map),
        expect![[r#"
            [
              [
                1,
                "1.2.3.4"
              ],
              [
                10,
                "1.2.3.4"
              ],
              [
                200,
                "255.255.255.255"
              ]
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SH(#[serde_as(as = "Vec<(DisplayFromStr, DisplayFromStr)>")] HashMap<u32, IpAddr>);

    // HashMap serialization tests with more than 1 entry are unreliable
    let map1: HashMap<_, _> = vec![(200, ip2)].into_iter().collect();
    let map: HashMap<_, _> = vec![(1, ip), (10, ip), (200, ip2)].into_iter().collect();
    is_equal(
        SH(map1.clone()),
        expect![[r#"
            [
              [
                "200",
                "255.255.255.255"
              ]
            ]"#]],
    );
    check_deserialization(
        SH(map.clone()),
        r#"[["1","1.2.3.4"],["10","1.2.3.4"],["200","255.255.255.255"]]"#,
    );
    check_error_deserialization::<SH>(
        r#"{"200":"255.255.255.255"}"#,
        expect![[r#"invalid type: map, expected a sequence at line 1 column 0"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SH2(#[serde_as(as = "Vec<(Same, DisplayFromStr)>")] HashMap<u32, IpAddr>);

    is_equal(
        SH2(map1),
        expect![[r#"
            [
              [
                200,
                "255.255.255.255"
              ]
            ]"#]],
    );
    check_deserialization(
        SH2(map),
        r#"[[1,"1.2.3.4"],[10,"1.2.3.4"],[200,"255.255.255.255"]]"#,
    );
    check_error_deserialization::<SH2>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected a sequence at line 1 column 1"#]],
    );
}

#[test]
fn test_tuple_list_as_map() {
    use std::{collections::HashMap, net::IpAddr};
    let ip = "1.2.3.4".parse().unwrap();
    let ip2 = "255.255.255.255".parse().unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SH(#[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")] Vec<(u32, IpAddr)>);

    is_equal(
        SH(vec![(1, ip), (10, ip), (200, ip2)]),
        expect![[r#"
            {
              "1": "1.2.3.4",
              "10": "1.2.3.4",
              "200": "255.255.255.255"
            }"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SB(#[serde_as(as = "BTreeMap<DisplayFromStr, DisplayFromStr>")] Vec<(u32, IpAddr)>);

    is_equal(
        SB(vec![(1, ip), (10, ip), (200, ip2)]),
        expect![[r#"
            {
              "1": "1.2.3.4",
              "10": "1.2.3.4",
              "200": "255.255.255.255"
            }"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SD(#[serde_as(as = "BTreeMap<DisplayFromStr, DisplayFromStr>")] VecDeque<(u32, IpAddr)>);

    is_equal(
        SD(vec![(1, ip), (10, ip), (200, ip2)].into()),
        expect![[r#"
            {
              "1": "1.2.3.4",
              "10": "1.2.3.4",
              "200": "255.255.255.255"
            }"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SLL(
        #[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")] LinkedList<(u32, IpAddr)>,
    );

    is_equal(
        SLL(vec![(1, ip), (10, ip), (200, ip2)].into_iter().collect()),
        expect![[r#"
            {
              "1": "1.2.3.4",
              "10": "1.2.3.4",
              "200": "255.255.255.255"
            }"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SO(#[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")] Option<(u32, IpAddr)>);

    is_equal(
        SO(Some((1, ip))),
        expect![[r#"
            {
              "1": "1.2.3.4"
            }"#]],
    );
    is_equal(SO(None), expect![[r#"{}"#]]);
}

#[test]
fn test_none_as_empty_string() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "NoneAsEmptyString")] Option<String>);

    is_equal(S(None), expect![[r#""""#]]);
    is_equal(S(Some("Hello".to_string())), expect![[r#""Hello""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SRc(#[serde_as(as = "NoneAsEmptyString")] Option<Rc<str>>);

    is_equal(SRc(None), expect![[r#""""#]]);
    is_equal(SRc(Some("Hello".into())), expect![[r#""Hello""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SArc(#[serde_as(as = "NoneAsEmptyString")] Option<Arc<str>>);

    is_equal(SArc(None), expect![[r#""""#]]);
    is_equal(SArc(Some("Hello".into())), expect![[r#""Hello""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SBox(#[serde_as(as = "NoneAsEmptyString")] Option<Box<str>>);

    is_equal(SBox(None), expect![[r#""""#]]);
    is_equal(SBox(Some("Hello".into())), expect![[r#""Hello""#]]);
}

#[test]
fn test_default_on_error() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "DefaultOnError<DisplayFromStr>")] u32);

    // Normal
    is_equal(S(123), expect![[r#""123""#]]);
    is_equal(S(0), expect![[r#""0""#]]);
    // Error cases
    check_deserialization(S(0), r#""""#);
    check_deserialization(S(0), r#""12+3""#);
    check_deserialization(S(0), r#""abc""#);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2(#[serde_as(as = "DefaultOnError<Vec<DisplayFromStr>>")] Vec<u32>);

    // Normal
    is_equal(
        S2(vec![1, 2, 3]),
        expect![[r#"
            [
              "1",
              "2",
              "3"
            ]"#]],
    );
    is_equal(S2(vec![]), expect![[r#"[]"#]]);
    // Error cases
    check_deserialization(S2(vec![]), r#"2"#);
    check_deserialization(S2(vec![]), r#""not_a_list""#);
    check_deserialization(S2(vec![]), r#"{}"#);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2 {
        #[serde_as(as = "DefaultOnError<Vec<DisplayFromStr>>")]
        value: Vec<u32>,
    };
    check_deserialization(Struct2 { value: vec![] }, r#"{"value":}"#);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S3(#[serde_as(as = "Vec<DefaultOnError<DisplayFromStr>>")] Vec<u32>);

    // Normal
    is_equal(
        S3(vec![1, 2, 3]),
        expect![[r#"
            [
              "1",
              "2",
              "3"
            ]"#]],
    );
    is_equal(S3(vec![]), expect![[r#"[]"#]]);
    // Error cases
    check_deserialization(S3(vec![0, 3, 0]), r#"[2,"3",4]"#);
    check_deserialization(S3(vec![0, 0]), r#"["AA",5]"#);
}

#[test]
fn test_bytes_or_string() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "BytesOrString")] Vec<u8>);

    is_equal(
        S(vec![1, 2, 3]),
        expect![[r#"
            [
              1,
              2,
              3
            ]"#]],
    );
    check_deserialization(S(vec![72, 101, 108, 108, 111]), r#""Hello""#);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SVec(#[serde_as(as = "Vec<BytesOrString>")] Vec<Vec<u8>>);

    is_equal(
        SVec(vec![vec![1, 2, 3]]),
        expect![[r#"
            [
              [
                1,
                2,
                3
              ]
            ]"#]],
    );
    check_deserialization(
        SVec(vec![
            vec![72, 101, 108, 108, 111],
            vec![87, 111, 114, 108, 100],
            vec![1, 2, 3],
        ]),
        r#"["Hello","World",[1,2,3]]"#,
    );
}

#[test]
fn test_duration_seconds() {
    use std::time::Duration;
    let zero = Duration::new(0, 0);
    let one_second = Duration::new(1, 0);
    let half_second = Duration::new(0, 500_000_000);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct IntStrict(#[serde_as(as = "DurationSeconds")] Duration);

    is_equal(IntStrict(zero), expect![[r#"0"#]]);
    is_equal(IntStrict(one_second), expect![[r#"1"#]]);
    check_serialization(IntStrict(half_second), expect![[r#"1"#]]);
    check_error_deserialization::<IntStrict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected u64 at line 1 column 3"#]],
    );
    check_error_deserialization::<IntStrict>(
        r#"-1"#,
        expect![[r#"invalid value: integer `-1`, expected u64 at line 1 column 2"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct IntFlexible(#[serde_as(as = "DurationSeconds<u64, Flexible>")] Duration);

    is_equal(IntFlexible(zero), expect![[r#"0"#]]);
    is_equal(IntFlexible(one_second), expect![[r#"1"#]]);
    check_serialization(IntFlexible(half_second), expect![[r#"1"#]]);
    check_deserialization(IntFlexible(half_second), r#""0.5""#);
    check_deserialization(IntFlexible(one_second), r#""1""#);
    check_deserialization(IntFlexible(zero), r#""0""#);
    check_error_deserialization::<IntFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
    check_error_deserialization::<IntFlexible>(
        r#"-1"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct F64Strict(#[serde_as(as = "DurationSeconds<f64>")] Duration);

    is_equal(F64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(F64Strict(one_second), expect![[r#"1.0"#]]);
    check_serialization(F64Strict(half_second), expect![[r#"1.0"#]]);
    check_deserialization(F64Strict(one_second), r#"0.5"#);
    check_error_deserialization::<F64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );
    check_error_deserialization::<F64Strict>(
        r#"-1.0"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct F64Flexible(#[serde_as(as = "DurationSeconds<f64, Flexible>")] Duration);

    is_equal(F64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(F64Flexible(one_second), expect![[r#"1.0"#]]);
    check_serialization(F64Flexible(half_second), expect![[r#"1.0"#]]);
    check_deserialization(F64Flexible(half_second), r#""0.5""#);
    check_deserialization(F64Flexible(one_second), r#""1""#);
    check_deserialization(F64Flexible(zero), r#""0""#);
    check_error_deserialization::<F64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
    check_error_deserialization::<F64Flexible>(
        r#"-1"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StringStrict(#[serde_as(as = "DurationSeconds<String>")] Duration);

    is_equal(StringStrict(zero), expect![[r#""0""#]]);
    is_equal(StringStrict(one_second), expect![[r#""1""#]]);
    check_serialization(StringStrict(half_second), expect![[r#""1""#]]);
    check_error_deserialization::<StringStrict>(
        r#"1"#,
        expect![[
            r#"invalid type: integer `1`, expected a string containing a number at line 1 column 1"#
        ]],
    );
    check_error_deserialization::<StringStrict>(
        r#"-1"#,
        expect![[
            r#"invalid type: integer `-1`, expected a string containing a number at line 1 column 2"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StringFlexible(#[serde_as(as = "DurationSeconds<String, Flexible>")] Duration);

    is_equal(StringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StringFlexible(one_second), expect![[r#""1""#]]);
    check_serialization(StringFlexible(half_second), expect![[r#""1""#]]);
    check_deserialization(StringFlexible(half_second), r#""0.5""#);
    check_deserialization(StringFlexible(one_second), r#""1""#);
    check_deserialization(StringFlexible(zero), r#""0""#);
    check_error_deserialization::<StringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
    check_error_deserialization::<StringFlexible>(
        r#"-1"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );
}

#[test]
fn test_duration_seconds_with_frac() {
    use std::time::Duration;
    let zero = Duration::new(0, 0);
    let one_second = Duration::new(1, 0);
    let half_second = Duration::new(0, 500_000_000);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct F64Strict(#[serde_as(as = "DurationSecondsWithFrac<f64>")] Duration);

    is_equal(F64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(F64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(F64Strict(half_second), expect![[r#"0.5"#]]);
    check_error_deserialization::<F64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );
    check_error_deserialization::<F64Strict>(
        r#"-1.0"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct F64Flexible(#[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")] Duration);

    is_equal(F64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(F64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(F64Flexible(half_second), expect![[r#"0.5"#]]);
    check_deserialization(F64Flexible(one_second), r#""1""#);
    check_deserialization(F64Flexible(zero), r#""0""#);
    check_error_deserialization::<F64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
    check_error_deserialization::<F64Flexible>(
        r#"-1"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StringStrict(#[serde_as(as = "DurationSecondsWithFrac<String>")] Duration);

    is_equal(StringStrict(zero), expect![[r#""0""#]]);
    is_equal(StringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StringStrict(half_second), expect![[r#""0.5""#]]);
    check_error_deserialization::<StringStrict>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 1"#]],
    );
    check_error_deserialization::<StringStrict>(
        r#"-1"#,
        expect![[r#"invalid type: integer `-1`, expected a string at line 1 column 2"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StringFlexible(#[serde_as(as = "DurationSecondsWithFrac<String, Flexible>")] Duration);

    is_equal(StringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StringFlexible(half_second), expect![[r#""0.5""#]]);
    check_deserialization(StringFlexible(zero), r#""0""#);
    check_error_deserialization::<StringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
    check_error_deserialization::<StringFlexible>(
        r#"-1"#,
        expect![[r#"std::time::Duration cannot be negative"#]],
    );
}

#[test]
fn string_with_separator() {
    use serde_with::{rust::StringWithSeparator, CommaSeparator, SpaceSeparator};

    #[serde_as]
    #[derive(Deserialize, Serialize)]
    struct A {
        #[serde_as(as = "StringWithSeparator::<SpaceSeparator, String>")]
        tags: Vec<String>,
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        // more_tags: Vec<String>,
        more_tags: BTreeSet<String>,
    }

    let v: A = serde_json::from_str(
        r##"{
    "tags": "#hello #world",
    "more_tags": "foo,bar,bar"
}"##,
    )
    .unwrap();
    assert_eq!(vec!["#hello", "#world"], v.tags);
    assert_eq!(2, v.more_tags.len());

    let x = A {
        tags: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        more_tags: Default::default(),
    };
    assert_eq!(
        r#"{"tags":"1 2 3","more_tags":""}"#,
        serde_json::to_string(&x).unwrap()
    );
}

#[test]
fn test_timestamp_seconds_systemtime() {
    let zero = SystemTime::UNIX_EPOCH;
    let one_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(1, 0))
        .unwrap();
    let half_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(0, 500_000_000))
        .unwrap();
    let minus_one_second = SystemTime::UNIX_EPOCH
        .checked_sub(Duration::new(1, 0))
        .unwrap();
    let minus_half_second = SystemTime::UNIX_EPOCH
        .checked_sub(Duration::new(0, 500_000_000))
        .unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntStrict(#[serde_as(as = "TimestampSeconds")] SystemTime);

    is_equal(StructIntStrict(zero), expect![[r#"0"#]]);
    is_equal(StructIntStrict(one_second), expect![[r#"1"#]]);
    is_equal(StructIntStrict(minus_one_second), expect![[r#"-1"#]]);
    check_serialization(StructIntStrict(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntStrict(minus_half_second), expect![[r#"-1"#]]);
    check_error_deserialization::<StructIntStrict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected i64 at line 1 column 3"#]],
    );
    check_error_deserialization::<StructIntStrict>(
        r#"0.123"#,
        expect![[r#"invalid type: floating point `0.123`, expected i64 at line 1 column 5"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructIntFlexible(#[serde_as(as = "TimestampSeconds<i64, Flexible>")] SystemTime);

    is_equal(StructIntFlexible(zero), expect![[r#"0"#]]);
    is_equal(StructIntFlexible(one_second), expect![[r#"1"#]]);
    is_equal(StructIntFlexible(minus_one_second), expect![[r#"-1"#]]);
    check_serialization(StructIntFlexible(half_second), expect![[r#"1"#]]);
    check_serialization(StructIntFlexible(minus_half_second), expect![[r#"-1"#]]);
    check_deserialization(StructIntFlexible(one_second), r#""1""#);
    check_deserialization(StructIntFlexible(one_second), r#"1.0"#);
    check_deserialization(StructIntFlexible(minus_half_second), r#""-0.5""#);
    check_deserialization(StructIntFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructIntFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "TimestampSeconds<f64>")] SystemTime);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Strict(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Strict(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Strict(one_second), r#"0.5"#);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(#[serde_as(as = "TimestampSeconds<f64, Flexible>")] SystemTime);

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    check_serialization(Structf64Flexible(half_second), expect![[r#"1.0"#]]);
    check_serialization(Structf64Flexible(minus_half_second), expect![[r#"-1.0"#]]);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(one_second), r#"1.0"#);
    check_deserialization(Structf64Flexible(minus_half_second), r#""-0.5""#);
    check_deserialization(Structf64Flexible(half_second), r#"0.5"#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "TimestampSeconds<String>")] SystemTime);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringStrict(half_second), expect![[r#""1""#]]);
    check_serialization(StructStringStrict(minus_half_second), expect![[r#""-1""#]]);
    check_deserialization(StructStringStrict(one_second), r#""1""#);
    check_error_deserialization::<StructStringStrict>(
        r#""0.5""#,
        expect![[r#"invalid digit found in string at line 1 column 5"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#""-0.5""#,
        expect![[r#"invalid digit found in string at line 1 column 6"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        expect![[
            r#"invalid type: integer `1`, expected a string containing a number at line 1 column 1"#
        ]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"0.0"#,
        expect![[
            r#"invalid type: floating point `0`, expected a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(#[serde_as(as = "TimestampSeconds<String, Flexible>")] SystemTime);

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    check_serialization(StructStringFlexible(half_second), expect![[r#""1""#]]);
    check_serialization(
        StructStringFlexible(minus_half_second),
        expect![[r#""-1""#]],
    );
    check_deserialization(StructStringFlexible(one_second), r#"1"#);
    check_deserialization(StructStringFlexible(one_second), r#"1.0"#);
    check_deserialization(StructStringFlexible(minus_half_second), r#""-0.5""#);
    check_deserialization(StructStringFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}

#[test]
fn test_timestamp_seconds_with_frac_systemtime() {
    let zero = SystemTime::UNIX_EPOCH;
    let one_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(1, 0))
        .unwrap();
    let half_second = SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(0, 500_000_000))
        .unwrap();
    let minus_one_second = SystemTime::UNIX_EPOCH
        .checked_sub(Duration::new(1, 0))
        .unwrap();
    let minus_half_second = SystemTime::UNIX_EPOCH
        .checked_sub(Duration::new(0, 500_000_000))
        .unwrap();

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Strict(#[serde_as(as = "TimestampSecondsWithFrac<f64>")] SystemTime);

    is_equal(Structf64Strict(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Strict(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Strict(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Strict(half_second), expect![[r#"0.5"#]]);
    is_equal(Structf64Strict(minus_half_second), expect![[r#"-0.5"#]]);
    check_error_deserialization::<Structf64Strict>(
        r#""1""#,
        expect![[r#"invalid type: string "1", expected f64 at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Structf64Flexible(
        #[serde_as(as = "TimestampSecondsWithFrac<f64, Flexible>")] SystemTime,
    );

    is_equal(Structf64Flexible(zero), expect![[r#"0.0"#]]);
    is_equal(Structf64Flexible(one_second), expect![[r#"1.0"#]]);
    is_equal(Structf64Flexible(minus_one_second), expect![[r#"-1.0"#]]);
    is_equal(Structf64Flexible(half_second), expect![[r#"0.5"#]]);
    is_equal(Structf64Flexible(minus_half_second), expect![[r#"-0.5"#]]);
    check_deserialization(Structf64Flexible(one_second), r#""1""#);
    check_deserialization(Structf64Flexible(one_second), r#"1.0"#);
    check_deserialization(Structf64Flexible(minus_half_second), r#""-0.5""#);
    check_deserialization(Structf64Flexible(half_second), r#"0.5"#);
    check_error_deserialization::<Structf64Flexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringStrict(#[serde_as(as = "TimestampSecondsWithFrac<String>")] SystemTime);

    is_equal(StructStringStrict(zero), expect![[r#""0""#]]);
    is_equal(StructStringStrict(one_second), expect![[r#""1""#]]);
    is_equal(StructStringStrict(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringStrict(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringStrict(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"1"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 1"#]],
    );
    check_error_deserialization::<StructStringStrict>(
        r#"0.0"#,
        expect![[r#"invalid type: floating point `0`, expected a string at line 1 column 3"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructStringFlexible(
        #[serde_as(as = "TimestampSecondsWithFrac<String, Flexible>")] SystemTime,
    );

    is_equal(StructStringFlexible(zero), expect![[r#""0""#]]);
    is_equal(StructStringFlexible(one_second), expect![[r#""1""#]]);
    is_equal(StructStringFlexible(minus_one_second), expect![[r#""-1""#]]);
    is_equal(StructStringFlexible(half_second), expect![[r#""0.5""#]]);
    is_equal(
        StructStringFlexible(minus_half_second),
        expect![[r#""-0.5""#]],
    );
    check_deserialization(StructStringFlexible(one_second), r#"1"#);
    check_deserialization(StructStringFlexible(one_second), r#"1.0"#);
    check_deserialization(StructStringFlexible(half_second), r#"0.5"#);
    check_error_deserialization::<StructStringFlexible>(
        r#""a""#,
        expect![[
            r#"invalid value: string "a", expected an integer, a float, or a string containing a number at line 1 column 3"#
        ]],
    );
}

#[test]
fn test_default_on_null() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "DefaultOnNull<DisplayFromStr>")] u32);

    // Normal
    is_equal(S(123), expect![[r#""123""#]]);
    is_equal(S(0), expect![[r#""0""#]]);
    // Null case
    check_deserialization(S(0), r#"null"#);
    // Error cases
    check_error_deserialization::<S>(
        r#""12+3""#,
        expect![[r#"invalid digit found in string at line 1 column 6"#]],
    );
    check_error_deserialization::<S>(
        r#""abc""#,
        expect![[r#"invalid digit found in string at line 1 column 5"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2(#[serde_as(as = "Vec<DefaultOnNull>")] Vec<u32>);

    // Normal
    is_equal(
        S2(vec![1, 2, 0, 3]),
        expect![[r#"
            [
              1,
              2,
              0,
              3
            ]"#]],
    );
    is_equal(S2(vec![]), expect![[r#"[]"#]]);
    // Null cases
    check_deserialization(S2(vec![1, 0, 2]), r#"[1, null, 2]"#);
    check_error_deserialization::<S2>(
        r#"["not_a_number"]"#,
        expect![[r#"invalid type: string "not_a_number", expected u32 at line 1 column 15"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S3(#[serde_as(as = "Vec<DefaultOnNull<DisplayFromStr>>")] Vec<u32>);

    // Normal
    is_equal(
        S3(vec![1, 2, 3]),
        expect![[r#"
            [
              "1",
              "2",
              "3"
            ]"#]],
    );
    // Null case
    check_deserialization(S3(vec![0, 3, 0]), r#"[null,"3",null]"#);
    check_error_deserialization::<S3>(
        r#"[null,3,null]"#,
        expect![[r#"invalid type: integer `3`, expected a string at line 1 column 7"#]],
    );
}
