mod utils;

use crate::utils::{check_deserialization, check_error_deserialization_expect, is_equal_expect};
use expect_test::expect;
use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use serde_derive::{Deserialize, Serialize};
use serde_with::CommaSeparator;
use std::{
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    iter::FromIterator as _,
};

#[test]
fn string_collection() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "serde_with::rust::StringWithSeparator::<CommaSeparator>")] Vec<String>,
    );

    is_equal_expect(S(vec![]), expect![[r#""""#]]);
    is_equal_expect(
        S(vec![
            "A".to_string(),
            "B".to_string(),
            "c".to_string(),
            "D".to_string(),
        ]),
        expect![[r#""A,B,c,D""#]],
    );
    is_equal_expect(
        S(vec!["".to_string(), "".to_string(), "".to_string()]),
        expect![[r#"",,""#]],
    );
    is_equal_expect(
        S(vec!["AVeryLongString".to_string()]),
        expect![[r#""AVeryLongString""#]],
    );
}

#[test]
fn prohibit_duplicate_value_hashset() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(#[serde(with = "::serde_with::rust::sets_duplicate_value_is_error")] HashSet<usize>);

    is_equal_expect(
        S(HashSet::from_iter(vec![1, 2, 3, 4])),
        expect![[r#"
            [
              4,
              1,
              3,
              2
            ]"#]],
    );
    check_error_deserialization_expect::<S>(
        r#"[1, 2, 3, 4, 1]"#,
        expect![[r#"invalid entry: found duplicate value at line 1 column 15"#]],
    );
}

#[test]
fn prohibit_duplicate_value_btreeset() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(#[serde(with = "::serde_with::rust::sets_duplicate_value_is_error")] BTreeSet<usize>);

    is_equal_expect(
        S(BTreeSet::from_iter(vec![1, 2, 3, 4])),
        expect![[r#"
            [
              1,
              2,
              3,
              4
            ]"#]],
    );
    check_error_deserialization_expect::<S>(
        r#"[1, 2, 3, 4, 1]"#,
        expect![[r#"invalid entry: found duplicate value at line 1 column 15"#]],
    );
}

#[test]
fn prohibit_duplicate_key_hashmap() {
    #[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")] HashMap<usize, usize>,
    );

    // Different value and key always works
    is_equal_expect(
        S(HashMap::from_iter(vec![(1, 1), (2, 2), (3, 3)])),
        expect![[r#"
            {
              "1": 1,
              "3": 3,
              "2": 2
            }"#]],
    );

    // Same value for different keys is ok
    is_equal_expect(
        S(HashMap::from_iter(vec![(1, 1), (2, 1), (3, 1)])),
        expect![[r#"
            {
              "1": 1,
              "3": 1,
              "2": 1
            }"#]],
    );

    // Duplicate keys are an error
    check_error_deserialization_expect::<S>(
        r#"{"1": 1, "2": 2, "1": 3}"#,
        expect![[r#"invalid entry: found duplicate key at line 1 column 24"#]],
    );
}

#[test]
fn prohibit_duplicate_key_btreemap() {
    #[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")] BTreeMap<usize, usize>,
    );

    // Different value and key always works
    is_equal_expect(
        S(BTreeMap::from_iter(vec![(1, 1), (2, 2), (3, 3)])),
        expect![[r#"
            {
              "1": 1,
              "2": 2,
              "3": 3
            }"#]],
    );

    // Same value for different keys is ok
    is_equal_expect(
        S(BTreeMap::from_iter(vec![(1, 1), (2, 1), (3, 1)])),
        expect![[r#"
            {
              "1": 1,
              "2": 1,
              "3": 1
            }"#]],
    );

    // Duplicate keys are an error
    check_error_deserialization_expect::<S>(
        r#"{"1": 1, "2": 2, "1": 3}"#,
        expect![[r#"invalid entry: found duplicate key at line 1 column 24"#]],
    );
}

#[test]
fn duplicate_key_first_wins_hashmap() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(#[serde(with = "::serde_with::rust::maps_first_key_wins")] HashMap<usize, usize>);

    // Different value and key always works
    is_equal_expect(
        S(HashMap::from_iter(vec![(1, 1), (2, 2), (3, 3)])),
        expect![[r#"
            {
              "1": 1,
              "3": 3,
              "2": 2
            }"#]],
    );

    // Same value for different keys is ok
    is_equal_expect(
        S(HashMap::from_iter(vec![(1, 1), (2, 1), (3, 1)])),
        expect![[r#"
            {
              "1": 1,
              "3": 1,
              "2": 1
            }"#]],
    );

    // Duplicate keys, the first one is used
    check_deserialization(
        S(HashMap::from_iter(vec![(1, 1), (2, 2)])),
        r#"{"1": 1, "2": 2, "1": 3}"#,
    );
}

#[test]
fn duplicate_key_first_wins_btreemap() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(#[serde(with = "::serde_with::rust::maps_first_key_wins")] BTreeMap<usize, usize>);

    // Different value and key always works
    is_equal_expect(
        S(BTreeMap::from_iter(vec![(1, 1), (2, 2), (3, 3)])),
        expect![[r#"
            {
              "1": 1,
              "2": 2,
              "3": 3
            }"#]],
    );

    // Same value for different keys is ok
    is_equal_expect(
        S(BTreeMap::from_iter(vec![(1, 1), (2, 1), (3, 1)])),
        expect![[r#"
            {
              "1": 1,
              "2": 1,
              "3": 1
            }"#]],
    );

    // Duplicate keys, the first one is used
    check_deserialization(
        S(BTreeMap::from_iter(vec![(1, 1), (2, 2)])),
        r#"{"1": 1, "2": 2, "1": 3}"#,
    );
}

#[test]
fn test_hashmap_as_tuple_list() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct S(#[serde(with = "serde_with::rust::hashmap_as_tuple_list")] HashMap<String, u8>);

    is_equal_expect(
        S(HashMap::from_iter(vec![
            ("ABC".to_string(), 1),
            ("Hello".to_string(), 0),
            ("World".to_string(), 20),
        ])),
        expect![[r#"
            [
              [
                "ABC",
                1
              ],
              [
                "Hello",
                0
              ],
              [
                "World",
                20
              ]
            ]"#]],
    );
    is_equal_expect(
        S(HashMap::from_iter(vec![("Hello".to_string(), 0)])),
        expect![[r#"
            [
              [
                "Hello",
                0
              ]
            ]"#]],
    );
    is_equal_expect(S(HashMap::default()), expect![[r#"[]"#]]);

    // Test parse error, only single element instead of tuple
    check_error_deserialization_expect::<S>(
        r#"[ [1] ]"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 4"#]],
    );
}

#[test]
fn test_btreemap_as_tuple_list() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct S(#[serde(with = "serde_with::rust::btreemap_as_tuple_list")] BTreeMap<String, u8>);

    is_equal_expect(
        S(BTreeMap::from_iter(vec![
            ("ABC".to_string(), 1),
            ("Hello".to_string(), 0),
            ("World".to_string(), 20),
        ])),
        expect![[r#"
            [
              [
                "ABC",
                1
              ],
              [
                "Hello",
                0
              ],
              [
                "World",
                20
              ]
            ]"#]],
    );
    is_equal_expect(
        S(BTreeMap::from_iter(vec![("Hello".to_string(), 0)])),
        expect![[r#"
            [
              [
                "Hello",
                0
              ]
            ]"#]],
    );
    is_equal_expect(S(BTreeMap::default()), expect![[r#"[]"#]]);

    // Test parse error, only single element instead of tuple
    check_error_deserialization_expect::<S>(
        r#"[ [1] ]"#,
        expect![[r#"invalid type: integer `1`, expected a string at line 1 column 4"#]],
    );
}

#[test]
fn tuple_list_as_map_vec() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "serde_with::rust::tuple_list_as_map")] Vec<(Wrapper<i32>, Wrapper<String>)>,
    );
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    struct Wrapper<T>(T);

    is_equal_expect(
        S(vec![
            (Wrapper(1), Wrapper("Hi".into())),
            (Wrapper(2), Wrapper("Cake".into())),
            (Wrapper(99), Wrapper("Lie".into())),
        ]),
        expect![[r#"
            {
              "1": "Hi",
              "2": "Cake",
              "99": "Lie"
            }"#]],
    );
    is_equal_expect(S(Vec::new()), expect![[r#"{}"#]]);
    check_error_deserialization_expect::<S>(
        r#"[]"#,
        expect![[r#"invalid type: sequence, expected a map at line 1 column 0"#]],
    );
    check_error_deserialization_expect::<S>(
        r#"null"#,
        expect![[r#"invalid type: null, expected a map at line 1 column 4"#]],
    );
}

#[test]
fn tuple_list_as_map_linkedlist() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "serde_with::rust::tuple_list_as_map")]
        LinkedList<(Wrapper<i32>, Wrapper<String>)>,
    );
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    struct Wrapper<T>(T);

    is_equal_expect(
        S(LinkedList::from_iter(vec![
            (Wrapper(1), Wrapper("Hi".into())),
            (Wrapper(2), Wrapper("Cake".into())),
            (Wrapper(99), Wrapper("Lie".into())),
        ])),
        expect![[r#"
            {
              "1": "Hi",
              "2": "Cake",
              "99": "Lie"
            }"#]],
    );
    is_equal_expect(S(LinkedList::new()), expect![[r#"{}"#]]);
    check_error_deserialization_expect::<S>(
        r#"[]"#,
        expect![[r#"invalid type: sequence, expected a map at line 1 column 0"#]],
    );
    check_error_deserialization_expect::<S>(
        r#"null"#,
        expect![[r#"invalid type: null, expected a map at line 1 column 4"#]],
    );
}

#[test]
fn tuple_list_as_map_vecdeque() {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct S(
        #[serde(with = "serde_with::rust::tuple_list_as_map")]
        VecDeque<(Wrapper<i32>, Wrapper<String>)>,
    );
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    struct Wrapper<T>(T);

    is_equal_expect(
        S(VecDeque::from_iter(vec![
            (Wrapper(1), Wrapper("Hi".into())),
            (Wrapper(2), Wrapper("Cake".into())),
            (Wrapper(99), Wrapper("Lie".into())),
        ])),
        expect![[r#"
            {
              "1": "Hi",
              "2": "Cake",
              "99": "Lie"
            }"#]],
    );
    is_equal_expect(S(VecDeque::new()), expect![[r#"{}"#]]);
    check_error_deserialization_expect::<S>(
        r#"[]"#,
        expect![[r#"invalid type: sequence, expected a map at line 1 column 0"#]],
    );
    check_error_deserialization_expect::<S>(
        r#"null"#,
        expect![[r#"invalid type: null, expected a map at line 1 column 4"#]],
    );
}
