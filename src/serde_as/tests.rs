use super::*;
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, fmt::Debug, rc::Rc, sync::Arc};

fn is_equal<T>(value: T, s: &str)
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
    struct Struct {
        #[serde(with = "As::<(DisplayFromStr, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct {
            values: (555_888, ip),
        },
        r#"{"values":["555888","1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct2 {
        #[serde(with = "As::<(SameAs<u32>, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct2 { values: (987, ip) },
        r#"{"values":[987,"1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct3 {
        #[serde(with = "As::<(Same, DisplayFromStr)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct3 { values: (987, ip) },
        r#"{"values":[987,"1.2.3.4"]}"#,
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
