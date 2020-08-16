use pretty_assertions::assert_eq;
use serde_with_macros::DeserializeFromStr;
use std::{
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

#[derive(Debug, PartialEq, DeserializeFromStr)]
struct A {
    a: u32,
    b: bool,
}

impl FromStr for A {
    type Err = String;

    /// Parse a value like `123<>true`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("<>");
        let number = parts
            .next()
            .ok_or_else(|| "Missing first value".to_string())?
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;
        let bool = parts
            .next()
            .ok_or_else(|| "Missing second value".to_string())?
            .parse()
            .map_err(|err: ParseBoolError| err.to_string())?;
        Ok(Self { a: number, b: bool })
    }
}

#[test]
fn test_deserialize_fromstr() {
    let a: A = serde_json::from_str("\"159<>true\"").unwrap();
    assert_eq!(A { a: 159, b: true }, a);
    let a: A = serde_json::from_str("\"999<>false\"").unwrap();
    assert_eq!(A { a: 999, b: false }, a);
    let a: A = serde_json::from_str("\"0<>true\"").unwrap();
    assert_eq!(A { a: 0, b: true }, a);
}

#[test]
fn test_deserialize_fromstr_in_vec() {
    let json = r#"[
  "123<>false",
  "0<>true",
  "999<>true"
]"#;
    let expected = vec![
        A { a: 123, b: false },
        A { a: 0, b: true },
        A { a: 999, b: true },
    ];
    assert_eq!(expected, serde_json::from_str::<Vec<A>>(&json).unwrap());
}
