// Nightly clippy contains upper_case_acronyms, unknown in older versions
// but clippy::unknown_clippy_lints is not accepted on nightly anymore (thus the other allows).
#![allow(unknown_lints, renamed_and_removed_lints, clippy::unknown_clippy_lints)]
#![allow(clippy::upper_case_acronyms)]

mod default_on;
mod map_tuple_list;
mod pickfirst;
mod serde_as_macro;
mod time;
#[path = "../utils.rs"]
mod utils;

use crate::utils::*;
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::{
    serde_as, BytesOrString, CommaSeparator, DisplayFromStr, NoneAsEmptyString, OneOrMany, Same,
    StringWithSeparator,
};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, LinkedList, VecDeque};
use std::rc::{Rc, Weak as RcWeak};
use std::sync::{Arc, Mutex, RwLock, Weak as ArcWeak};

#[test]
fn test_basic_wrappers() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SBox(#[serde_as(as = "Box<DisplayFromStr>")] Box<u32>);

    is_equal(SBox(Box::new(123)), expect![[r#""123""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SRc(#[serde_as(as = "Rc<DisplayFromStr>")] Rc<u32>);

    is_equal(SRc(Rc::new(123)), expect![[r#""123""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    struct SRcWeak(#[serde_as(as = "RcWeak<DisplayFromStr>")] RcWeak<u32>);

    check_serialization(SRcWeak(RcWeak::new()), expect![[r#"null"#]]);
    let s: SRcWeak = serde_json::from_str("null").unwrap();
    assert!(s.0.upgrade().is_none());
    let s: SRcWeak = serde_json::from_str("\"123\"").unwrap();
    assert!(s.0.upgrade().is_none());

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SArc(#[serde_as(as = "Arc<DisplayFromStr>")] Arc<u32>);

    is_equal(SArc(Arc::new(123)), expect![[r#""123""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    struct SArcWeak(#[serde_as(as = "ArcWeak<DisplayFromStr>")] ArcWeak<u32>);

    check_serialization(SArcWeak(ArcWeak::new()), expect![[r#"null"#]]);
    let s: SArcWeak = serde_json::from_str("null").unwrap();
    assert!(s.0.upgrade().is_none());
    let s: SArcWeak = serde_json::from_str("\"123\"").unwrap();
    assert!(s.0.upgrade().is_none());

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SCell(#[serde_as(as = "Cell<DisplayFromStr>")] Cell<u32>);

    is_equal(SCell(Cell::new(123)), expect![[r#""123""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SRefCell(#[serde_as(as = "RefCell<DisplayFromStr>")] RefCell<u32>);

    is_equal(SRefCell(RefCell::new(123)), expect![[r#""123""#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    struct SMutex(#[serde_as(as = "Mutex<DisplayFromStr>")] Mutex<u32>);

    check_serialization(SMutex(Mutex::new(123)), expect![[r#""123""#]]);
    let s: SMutex = serde_json::from_str("\"123\"").unwrap();
    assert_eq!(*s.0.lock().unwrap(), 123);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    struct SRwLock(#[serde_as(as = "RwLock<DisplayFromStr>")] RwLock<u32>);

    let expected = expect![[r#""123""#]];
    check_serialization(SRwLock(RwLock::new(123)), expected);
    let s: SRwLock = serde_json::from_str("\"123\"").unwrap();
    assert_eq!(*s.0.read().unwrap(), 123);
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

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "_")]
        value: Option<u32>,
    }
    check_error_deserialization::<Struct>(
        r#"{}"#,
        expect![[r#"missing field `value` at line 1 column 2"#]],
    );
}

#[test]
fn test_result() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(
        #[serde_as(as = "Result<StringWithSeparator<CommaSeparator, u32>, DisplayFromStr>")]
        Result<Vec<u32>, u32>,
    );

    is_equal(
        S(Ok(vec![1, 2, 3])),
        expect![[r#"
        {
          "Ok": "1,2,3"
        }"#]],
    );
    is_equal(
        S(Err(9)),
        expect![[r#"
            {
              "Err": "9"
            }"#]],
    );
    check_error_deserialization::<S>(r#"{}"#, expect![[r#"expected value at line 1 column 2"#]]);
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
fn string_with_separator() {
    use serde_with::rust::StringWithSeparator;
    use serde_with::{CommaSeparator, SpaceSeparator};

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
fn test_serialize_reference() {
    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S1<'a>(#[serde_as(as = "Vec<DisplayFromStr>")] &'a Vec<u32>);
    check_serialization(
        S1(&vec![1, 2]),
        expect![[r#"
        [
          "1",
          "2"
        ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S1a<'a>(#[serde_as(as = "&Vec<DisplayFromStr>")] &'a Vec<u32>);
    check_serialization(
        S1(&vec![1, 2]),
        expect![[r#"
        [
          "1",
          "2"
        ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S1Mut<'a>(#[serde_as(as = "Vec<DisplayFromStr>")] &'a mut Vec<u32>);
    check_serialization(
        S1(&vec![1, 2]),
        expect![[r#"
        [
          "1",
          "2"
        ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S1aMut<'a>(#[serde_as(as = "&mut Vec<DisplayFromStr>")] &'a mut Vec<u32>);
    check_serialization(
        S1(&vec![1, 2]),
        expect![[r#"
        [
          "1",
          "2"
        ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S2<'a>(#[serde_as(as = "&[DisplayFromStr]")] &'a [u32]);
    check_serialization(
        S2(&[1, 2]),
        expect![[r#"
            [
              "1",
              "2"
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S3<'a>(
        #[serde_as(as = "&BTreeMap<DisplayFromStr, DisplayFromStr>")] &'a BTreeMap<bool, u32>,
    );
    let bmap = vec![(false, 123), (true, 456)].into_iter().collect();
    check_serialization(
        S3(&bmap),
        expect![[r#"
            {
              "false": "123",
              "true": "456"
            }"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S4<'a>(#[serde_as(as = "&Vec<(_, DisplayFromStr)>")] &'a BTreeMap<bool, u32>);
    let bmap = vec![(false, 123), (true, 456)].into_iter().collect();
    check_serialization(
        S4(&bmap),
        expect![[r#"
            [
              [
                false,
                "123"
              ],
              [
                true,
                "456"
              ]
            ]"#]],
    );

    #[serde_as]
    #[derive(Debug, Serialize)]
    struct S5<'a>(
        #[serde_as(as = "&BTreeMap<DisplayFromStr, &Vec<(_, _)>>")]
        &'a Vec<(u32, &'a BTreeMap<bool, String>)>,
    );
    let bmap0 = vec![(false, "123".to_string()), (true, "456".to_string())]
        .into_iter()
        .collect();
    let bmap1 = vec![(true, "Hello".to_string()), (false, "World".to_string())]
        .into_iter()
        .collect();
    let vec = vec![(111, &bmap0), (999, &bmap1)];
    check_serialization(
        S5(&vec),
        expect![[r#"
            {
              "111": [
                [
                  false,
                  "123"
                ],
                [
                  true,
                  "456"
                ]
              ],
              "999": [
                [
                  false,
                  "World"
                ],
                [
                  true,
                  "Hello"
                ]
              ]
            }"#]],
    );
}

#[rustversion::since(1.51)]
#[test]
fn test_big_arrays() {
    // Single Big Array
    #[serde_as]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct S1(#[serde_as(as = "[_; 64]")] [u8; 64]);
    is_equal_compact(
        S1([0; 64]),
        expect![[
            r#"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        ]],
    );
    // Too few entries
    check_error_deserialization::<S1>(
        r#"[0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#,
        expect![[r#"invalid length 14, expected an array of size 64 at line 1 column 29"#]],
    );
    // Too many entries
    check_error_deserialization::<S1>(
        r#"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#,
        expect![[r#"trailing characters at line 1 column 130"#]],
    );

    // Single Big Array
    #[serde_as]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct S2(#[serde_as(as = "[DisplayFromStr; 40]")] [u8; 40]);
    is_equal_compact(
        S2([0; 40]),
        expect![[
            r#"["0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0"]"#
        ]],
    );

    // Nested Big Arrays
    #[serde_as]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct S3(#[serde_as(as = "[[_; 34]; 33]")] [[u8; 34]; 33]);
    is_equal_compact(
        S3([[0; 34]; 33]),
        expect![[
            r#"[[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]"#
        ]],
    );
}

// The test requires const-generics to work
#[rustversion::since(1.51)]
#[test]
fn test_bytes() {
    // The test case is copied from
    // https://github.com/serde-rs/bytes/blob/cbae606b9dc225fc094b031cc84eac9493da2058/tests/test_derive.rs
    // Original code by @dtolnay

    use serde_test::{assert_de_tokens, assert_tokens, Token};
    use serde_with::Bytes;
    use std::borrow::Cow;

    #[serde_as]
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Test<'a> {
        #[serde_as(as = "Bytes")]
        array: [u8; 52],

        #[serde_as(as = "Bytes")]
        slice: &'a [u8],

        #[serde_as(as = "Bytes")]
        vec: Vec<u8>,

        #[serde_as(as = "Bytes")]
        cow_slice: Cow<'a, [u8]>,

        #[serde_as(as = "Box<Bytes>")]
        boxed_array: Box<[u8; 52]>,

        #[serde_as(as = "Bytes")]
        boxed_array2: Box<[u8; 52]>,

        #[serde_as(as = "Bytes")]
        boxed_slice: Box<[u8]>,

        #[serde_as(as = "Option<Bytes>")]
        opt_slice: Option<&'a [u8]>,

        #[serde_as(as = "Option<Bytes>")]
        opt_vec: Option<Vec<u8>>,

        #[serde_as(as = "Option<Bytes>")]
        opt_cow_slice: Option<Cow<'a, [u8]>>,
    }

    let test = Test {
        array: *b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz",
        slice: b"...",
        vec: b"...".to_vec(),
        cow_slice: Cow::Borrowed(b"..."),
        boxed_array: Box::new(*b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
        boxed_array2: Box::new(*b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
        boxed_slice: b"...".to_vec().into_boxed_slice(),
        opt_slice: Some(b"..."),
        opt_vec: Some(b"...".to_vec()),
        opt_cow_slice: Some(Cow::Borrowed(b"...")),
    };

    assert_tokens(
        &test,
        &[
            Token::Struct {
                name: "Test",
                len: 10,
            },
            Token::Str("array"),
            Token::BorrowedBytes(b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("slice"),
            Token::BorrowedBytes(b"..."),
            Token::Str("vec"),
            Token::Bytes(b"..."),
            Token::Str("cow_slice"),
            Token::BorrowedBytes(b"..."),
            Token::Str("boxed_array"),
            Token::BorrowedBytes(b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("boxed_array2"),
            Token::BorrowedBytes(b"ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("boxed_slice"),
            Token::Bytes(b"..."),
            Token::Str("opt_slice"),
            Token::Some,
            Token::BorrowedBytes(b"..."),
            Token::Str("opt_vec"),
            Token::Some,
            Token::Bytes(b"..."),
            Token::Str("opt_cow_slice"),
            Token::Some,
            Token::BorrowedBytes(b"..."),
            Token::StructEnd,
        ],
    );

    // Test string deserialization
    assert_de_tokens(
        &test,
        &[
            Token::Struct {
                name: "Test",
                len: 10,
            },
            Token::Str("array"),
            Token::BorrowedStr("ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("slice"),
            Token::BorrowedStr("..."),
            Token::Str("vec"),
            Token::Bytes(b"..."),
            Token::Str("cow_slice"),
            Token::BorrowedStr("..."),
            Token::Str("boxed_array"),
            Token::BorrowedStr("ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("boxed_array2"),
            Token::BorrowedStr("ABCDEFGHIJKLMNOPQRSTUVWXZYabcdefghijklmnopqrstuvwxyz"),
            Token::Str("boxed_slice"),
            Token::Bytes(b"..."),
            Token::Str("opt_slice"),
            Token::Some,
            Token::BorrowedStr("..."),
            Token::Str("opt_vec"),
            Token::Some,
            Token::Bytes(b"..."),
            Token::Str("opt_cow_slice"),
            Token::Some,
            Token::BorrowedStr("..."),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_one_or_many_prefer_one() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S1Vec(#[serde_as(as = "OneOrMany<_>")] Vec<u32>);

    // Normal
    is_equal(S1Vec(vec![]), expect![[r#"[]"#]]);
    is_equal(S1Vec(vec![1]), expect![[r#"1"#]]);
    is_equal(
        S1Vec(vec![1, 2, 3]),
        expect![[r#"
            [
              1,
              2,
              3
            ]"#]],
    );
    check_deserialization(S1Vec(vec![1]), r#"1"#);
    check_deserialization(S1Vec(vec![1]), r#"[1]"#);
    check_error_deserialization::<S1Vec>(r#"{}"#, expect![[r#"a list or single element"#]]);
    check_error_deserialization::<S1Vec>(r#""xx""#, expect![[r#"a list or single element"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2Vec(#[serde_as(as = "OneOrMany<DisplayFromStr>")] Vec<u32>);

    // Normal
    is_equal(S2Vec(vec![]), expect![[r#"[]"#]]);
    is_equal(S2Vec(vec![1]), expect![[r#""1""#]]);
    is_equal(
        S2Vec(vec![1, 2, 3]),
        expect![[r#"
            [
              "1",
              "2",
              "3"
            ]"#]],
    );
    check_deserialization(S2Vec(vec![1]), r#""1""#);
    check_deserialization(S2Vec(vec![1]), r#"["1"]"#);
    check_error_deserialization::<S2Vec>(r#"{}"#, expect![[r#"a list or single element"#]]);
    check_error_deserialization::<S2Vec>(r#""xx""#, expect![[r#"a list or single element"#]]);
}

#[test]
fn test_one_or_many_prefer_many() {
    use serde_with::formats::PreferMany;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S1Vec(#[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<u32>);

    // Normal
    is_equal(S1Vec(vec![]), expect![[r#"[]"#]]);
    is_equal(
        S1Vec(vec![1]),
        expect![[r#"
            [
              1
            ]"#]],
    );
    is_equal(
        S1Vec(vec![1, 2, 3]),
        expect![[r#"
            [
              1,
              2,
              3
            ]"#]],
    );
    check_deserialization(S1Vec(vec![1]), r#"1"#);
    check_deserialization(S1Vec(vec![1]), r#"[1]"#);
    check_error_deserialization::<S1Vec>(r#"{}"#, expect![[r#"a list or single element"#]]);
    check_error_deserialization::<S1Vec>(r#""xx""#, expect![[r#"a list or single element"#]]);

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S2Vec(#[serde_as(as = "OneOrMany<DisplayFromStr, PreferMany>")] Vec<u32>);

    // Normal
    is_equal(S2Vec(vec![]), expect![[r#"[]"#]]);
    is_equal(
        S2Vec(vec![1]),
        expect![[r#"
            [
              "1"
            ]"#]],
    );
    is_equal(
        S2Vec(vec![1, 2, 3]),
        expect![[r#"
            [
              "1",
              "2",
              "3"
            ]"#]],
    );
    check_deserialization(S2Vec(vec![1]), r#""1""#);
    check_deserialization(S2Vec(vec![1]), r#"["1"]"#);
    check_error_deserialization::<S2Vec>(r#"{}"#, expect![[r#"a list or single element"#]]);
    check_error_deserialization::<S2Vec>(r#""xx""#, expect![[r#"a list or single element"#]]);
}
