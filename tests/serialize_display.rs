use pretty_assertions::assert_eq;
use serde_with::SerializeDisplay;
use std::fmt;

#[derive(SerializeDisplay)]
struct A {
    a: u32,
    b: bool,
}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "->{} <> {}<-", self.a, self.b)
    }
}

#[test]
fn test_serialize_display() {
    let a = A { a: 123, b: false };
    assert_eq!(r#""->123 <> false<-""#, serde_json::to_string(&a).unwrap());
    let a = A { a: 0, b: true };
    assert_eq!(r#""->0 <> true<-""#, serde_json::to_string(&a).unwrap());
    let a = A { a: 999, b: true };
    assert_eq!(r#""->999 <> true<-""#, serde_json::to_string(&a).unwrap());
}

#[test]
fn test_serialize_display_in_vec() {
    let v = vec![
        A { a: 123, b: false },
        A { a: 0, b: true },
        A { a: 999, b: true },
    ];
    let expected = r#"[
  "->123 <> false<-",
  "->0 <> true<-",
  "->999 <> true<-"
]"#;
    assert_eq!(expected, serde_json::to_string_pretty(&v).unwrap());
}
