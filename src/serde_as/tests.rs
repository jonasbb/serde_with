use super::*;
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, fmt::Debug, rc::Rc, sync::Arc};

pub(crate) fn is_equal<T>(value: T, s: &str)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    assert_eq!(
        serde_json::from_str::<T>(s).unwrap(),
        value,
        "Deserialization differs from expected value."
    );
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        s,
        "Serialization differs from expected value."
    );
}

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
