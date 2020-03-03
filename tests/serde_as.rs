mod utils;

use crate::utils::{check_deserialization, is_equal};
use serde::{Deserialize, Serialize};
use serde_with::{
    As, BytesOrString, DefaultOnError, DisplayFromStr, NoneAsEmptyString, Same, SameAs,
};
use std::{collections::BTreeMap, fmt::Debug, rc::Rc, sync::Arc};

#[test]
fn test_display_fromstr() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<DisplayFromStr>")]
        value: u32,
    };

    is_equal(Struct { value: 123 }, r#"{"value":"123"}"#);
}

#[test]
fn test_tuples() {
    use std::net::IpAddr;
    let ip = "1.2.3.4".parse().unwrap();

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct1 {
        #[serde(with = "As::<(DisplayFromStr,)>")]
        values: (u32,),
    };
    is_equal(Struct1 { values: (1,) }, r#"{"values":["1"]}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2a {
        #[serde(with = "As::<(DisplayFromStr, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct2a {
            values: (555_888, ip),
        },
        r#"{"values":["555888","1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2b {
        #[serde(with = "As::<(SameAs<u32>, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct2b { values: (987, ip) },
        r#"{"values":[987,"1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2c {
        #[serde(with = "As::<(Same, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct2c { values: (987, ip) },
        r#"{"values":[987,"1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct6 {
        #[serde(with = "As::<(Same, Same, Same, Same, Same, Same)>")]
        values: (u8, u16, u32, i8, i16, i32),
    };
    is_equal(
        Struct6 {
            values: (8, 16, 32, -8, 16, -32),
        },
        r#"{"values":[8,16,32,-8,16,-32]}"#,
    );
}

#[test]
fn test_arrays() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct0a {
        #[serde(with = "As::<[DisplayFromStr; 0]>")]
        values: [u32; 0],
    };
    is_equal(Struct0a { values: [] }, r#"{"values":[]}"#);

    // Test "non-matching" types.
    // Arrays of size 0 should allow all convertions
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct0b {
        #[serde(with = "As::<[u8; 0]>")]
        values: [String; 0],
    };
    is_equal(Struct0b { values: [] }, r#"{"values":[]}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct1 {
        #[serde(with = "As::<[DisplayFromStr; 1]>")]
        values: [u32; 1],
    };
    is_equal(Struct1 { values: [1] }, r#"{"values":["1"]}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2 {
        #[serde(with = "As::<[Same; 2]>")]
        values: [u32; 2],
    };
    is_equal(Struct2 { values: [11, 22] }, r#"{"values":[11,22]}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct32 {
        #[serde(with = "As::<[Same; 32]>")]
        values: [u32; 32],
    };
    is_equal(
        Struct32 {
            values: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
        },
        r#"{"values":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]}"#,
    );
}

#[test]
fn test_map_as_tuple_list() {
    use std::net::IpAddr;
    let ip = "1.2.3.4".parse().unwrap();
    let ip2 = "255.255.255.255".parse().unwrap();

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<Vec<(DisplayFromStr, DisplayFromStr)>>")]
        values: BTreeMap<u32, IpAddr>,
    };

    let map: BTreeMap<_, _> = vec![(1, ip), (10, ip), (200, ip2)].into_iter().collect();
    is_equal(
        Struct {
            values: map.clone(),
        },
        r#"{"values":[["1","1.2.3.4"],["10","1.2.3.4"],["200","255.255.255.255"]]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2 {
        #[serde(with = "As::<Vec<(Same, DisplayFromStr)>>")]
        values: BTreeMap<u32, IpAddr>,
    };

    is_equal(
        Struct2 { values: map },
        r#"{"values":[[1,"1.2.3.4"],[10,"1.2.3.4"],[200,"255.255.255.255"]]}"#,
    );
}

#[test]
fn test_tuple_list_as_map() {
    use std::{collections::HashMap, net::IpAddr};
    let ip = "1.2.3.4".parse().unwrap();
    let ip2 = "255.255.255.255".parse().unwrap();

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructHashMap {
        #[serde(with = "As::<HashMap<DisplayFromStr, DisplayFromStr>>")]
        values: Vec<(u32, IpAddr)>,
    };

    is_equal(
        StructHashMap {
            values: vec![(1, ip), (10, ip), (200, ip2)],
        },
        r#"{"values":{"1":"1.2.3.4","10":"1.2.3.4","200":"255.255.255.255"}}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructBTreeMap {
        #[serde(with = "As::<BTreeMap<DisplayFromStr, DisplayFromStr>>")]
        values: Vec<(u32, IpAddr)>,
    };

    is_equal(
        StructBTreeMap {
            values: vec![(1, ip), (10, ip), (200, ip2)],
        },
        r#"{"values":{"1":"1.2.3.4","10":"1.2.3.4","200":"255.255.255.255"}}"#,
    );

    // #[derive(Debug, Serialize, Deserialize, PartialEq)]
    // struct StructDeque {
    //     #[serde(with = "As::<BTreeMap<DisplayFromStr, DisplayFromStr>>")]
    //     values: VecDeque<(u32, IpAddr)>,
    // };

    // is_equal(
    //     StructDeque {
    //         values: vec![(1, ip), (10, ip), (200, ip2)].into(),
    //     },
    //     r#"{"values":{"1":"1.2.3.4","10":"1.2.3.4","200":"255.255.255.255"}}"#,
    // );
}

#[test]
fn test_none_as_empty_string() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<NoneAsEmptyString>")]
        value: Option<String>,
    };

    is_equal(Struct { value: None }, r#"{"value":""}"#);
    is_equal(
        Struct {
            value: Some("Hello".to_string()),
        },
        r#"{"value":"Hello"}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructRc {
        #[serde(with = "As::<NoneAsEmptyString>")]
        value: Option<Rc<str>>,
    };

    is_equal(StructRc { value: None }, r#"{"value":""}"#);
    is_equal(
        StructRc {
            value: Some("Hello".into()),
        },
        r#"{"value":"Hello"}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructArc {
        #[serde(with = "As::<NoneAsEmptyString>")]
        value: Option<Arc<str>>,
    };

    is_equal(StructArc { value: None }, r#"{"value":""}"#);
    is_equal(
        StructArc {
            value: Some("Hello".into()),
        },
        r#"{"value":"Hello"}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructBox {
        #[serde(with = "As::<NoneAsEmptyString>")]
        value: Option<Box<str>>,
    };

    is_equal(StructBox { value: None }, r#"{"value":""}"#);
    is_equal(
        StructBox {
            value: Some("Hello".into()),
        },
        r#"{"value":"Hello"}"#,
    );
}

#[test]
fn test_default_on_error() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<DefaultOnError<DisplayFromStr>>")]
        value: u32,
    };

    // Normal
    is_equal(Struct { value: 123 }, r#"{"value":"123"}"#);
    is_equal(Struct { value: 0 }, r#"{"value":"0"}"#);
    // Error cases
    check_deserialization(Struct { value: 0 }, r#"{"value":""}"#);
    check_deserialization(Struct { value: 0 }, r#"{"value":"12+3"}"#);
    check_deserialization(Struct { value: 0 }, r#"{"value":"abc"}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2 {
        #[serde(with = "As::<DefaultOnError<Vec<DisplayFromStr>>>")]
        value: Vec<u32>,
    };

    // Normal
    is_equal(
        Struct2 {
            value: vec![1, 2, 3],
        },
        r#"{"value":["1","2","3"]}"#,
    );
    is_equal(Struct2 { value: vec![] }, r#"{"value":[]}"#);
    // Error cases
    check_deserialization(Struct2 { value: vec![] }, r#"{"value":2}"#);
    check_deserialization(Struct2 { value: vec![] }, r#"{"value":"notalist"}"#);
    // TODO why does this result in
    // thread 'test_default_on_error' panicked at 'called `Result::unwrap()` on an `Err` value: Error("expected `,` or `}`", line: 1, column: 10)', tests/utils.rs:32:9
    // check_deserialization(Struct2 { value: vec![] }, r#"{"value":{}}"#);
    check_deserialization(Struct2 { value: vec![] }, r#"{"value":}"#);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct3 {
        #[serde(with = "As::<Vec<DefaultOnError<DisplayFromStr>>>")]
        value: Vec<u32>,
    };

    // Normal
    is_equal(
        Struct3 {
            value: vec![1, 2, 3],
        },
        r#"{"value":["1","2","3"]}"#,
    );
    is_equal(Struct3 { value: vec![] }, r#"{"value":[]}"#);
    // Error cases
    check_deserialization(
        Struct3 {
            value: vec![0, 0, 0],
        },
        r#"{"value":[2,3,4]}"#,
    );
    check_deserialization(Struct3 { value: vec![0, 0] }, r#"{"value":["AA",5]}"#);
}

#[test]
fn test_bytes_or_string() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde(with = "As::<BytesOrString>")]
        value: Vec<u8>,
    };

    is_equal(
        Struct {
            value: vec![1, 2, 3],
        },
        r#"{"value":[1,2,3]}"#,
    );
    check_deserialization(
        Struct {
            value: vec![72, 101, 108, 108, 111],
        },
        r#"{"value":"Hello"}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StructVec {
        #[serde(with = "As::<Vec<BytesOrString>>")]
        value: Vec<Vec<u8>>,
    };

    is_equal(
        StructVec {
            value: vec![vec![1, 2, 3]],
        },
        r#"{"value":[[1,2,3]]}"#,
    );
    check_deserialization(
        StructVec {
            value: vec![
                vec![72, 101, 108, 108, 111],
                vec![87, 111, 114, 108, 100],
                vec![1, 2, 3],
            ],
        },
        r#"{"value":["Hello","World",[1,2,3]]}"#,
    );
}
