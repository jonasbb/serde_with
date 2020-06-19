use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Test that the [`serde_as`] macro can replace the `_` type and the resulting code compiles.
#[test]
fn test_serde_as_macro_replace_infer_type() {
    #[serde_as]
    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Data {
        #[serde_as(as = "_")]
        a: u32,
        #[serde_as(as = "std::vec::Vec<_>")]
        b: Vec<u32>,
        #[serde_as(as = "Vec<(_, _)>")]
        c: Vec<(u32, String)>,
        #[serde_as(as = "[_; 2]")]
        d: [u32; 2],
        #[serde_as(as = "Box<[_]>")]
        e: Box<[u32]>,
    }

    let data = Data {
        a: 10,
        b: vec![20, 33],
        c: vec![(40, "Hello".into()), (55, "World".into()), (60, "!".into())],
        d: [70, 88],
        e: vec![99, 100, 110].into_boxed_slice(),
    };
    let expected = r##"{
  "a": 10,
  "b": [
    20,
    33
  ],
  "c": [
    [
      40,
      "Hello"
    ],
    [
      55,
      "World"
    ],
    [
      60,
      "!"
    ]
  ],
  "d": [
    70,
    88
  ],
  "e": [
    99,
    100,
    110
  ]
}"##;

    assert_eq!(expected, serde_json::to_string_pretty(&data).unwrap());
    assert_eq!(data, serde_json::from_str(expected).unwrap());
}

/// Test that the [`serde_as`] macro supports `deserialize_as`
#[test]
fn test_serde_as_macro_deserialize() {
    #[serde_as]
    #[derive(Debug, Eq, PartialEq, Deserialize)]
    struct Data {
        #[serde_as(deserialize_as = "DisplayFromStr")]
        a: u32,
        #[serde_as(deserialize_as = "Vec<DisplayFromStr>")]
        b: Vec<u32>,
        #[serde_as(deserialize_as = "(DisplayFromStr, _)")]
        c: (u32, u32),
    }

    let data = Data {
        a: 10,
        b: vec![20, 33],
        c: (40, 55),
    };
    let expected = r##"{
  "a": "10",
  "b": [
    "20",
    "33"
  ],
  "c": [
    "40",
    55
  ]
}"##;

    assert_eq!(data, serde_json::from_str(expected).unwrap());
}

/// Test that the [`serde_as`] macro supports `serialize_as`
#[test]
fn test_serde_as_macro_serialize() {
    #[serde_as]
    #[derive(Debug, Eq, PartialEq, Serialize)]
    struct Data {
        #[serde_as(serialize_as = "DisplayFromStr")]
        a: u32,
        #[serde_as(serialize_as = "Vec<DisplayFromStr>")]
        b: Vec<u32>,
        #[serde_as(serialize_as = "(DisplayFromStr, _)")]
        c: (u32, u32),
    }

    let data = Data {
        a: 10,
        b: vec![20, 33],
        c: (40, 55),
    };
    let expected = r##"{
  "a": "10",
  "b": [
    "20",
    "33"
  ],
  "c": [
    "40",
    55
  ]
}"##;

    assert_eq!(expected, serde_json::to_string_pretty(&data).unwrap());
}

/// Test that the [`serde_as`] macro supports `serialize_as` and `deserialize_as`
#[test]
fn test_serde_as_macro_serialize_deserialize() {
    #[serde_as]
    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Data {
        #[serde_as(serialize_as = "DisplayFromStr", deserialize_as = "DisplayFromStr")]
        a: u32,
        #[serde_as(
            serialize_as = "Vec<DisplayFromStr>",
            deserialize_as = "Vec<DisplayFromStr>"
        )]
        b: Vec<u32>,
        #[serde_as(
            serialize_as = "(DisplayFromStr, _)",
            deserialize_as = "(DisplayFromStr, _)"
        )]
        c: (u32, u32),
    }

    let data = Data {
        a: 10,
        b: vec![20, 33],
        c: (40, 55),
    };
    let expected = r##"{
  "a": "10",
  "b": [
    "20",
    "33"
  ],
  "c": [
    "40",
    55
  ]
}"##;

    assert_eq!(expected, serde_json::to_string_pretty(&data).unwrap());
    assert_eq!(data, serde_json::from_str(expected).unwrap());
}
