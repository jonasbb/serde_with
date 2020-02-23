use super::*;
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, fmt::Debug};

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
        #[serde(with = "As::<DisplayString>")]
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
        #[serde(with = "As::<(DisplayString, DisplayString)>")]
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
        #[serde(with = "As::<(SameAs<u32>, DisplayString)>")]
        values: (u32, IpAddr),
    };
    is_equal(
        Struct2 { values: (987, ip) },
        r#"{"values":[987,"1.2.3.4"]}"#,
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct3 {
        #[serde(with = "As::<(Same, DisplayString)>")]
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
        #[serde(with = "As::<Vec<(DisplayString, DisplayString)>>")]
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
        #[serde(with = "As::<Vec<(Same, DisplayString)>>")]
        values: BTreeMap<u32, IpAddr>,
    };

    is_equal(
        Struct2 { values: map },
        r#"{"values":[[1,"1.2.3.4"],[10,"1.2.3.4"],[200,"255.255.255.255"]]}"#,
    );
}
