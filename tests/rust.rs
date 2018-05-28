extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_with;
#[macro_use]
extern crate pretty_assertions;

use serde_with::CommaSeparator;

#[test]
fn string_collection() {
    #[derive(Debug, Deserialize)]
    struct S {
        #[serde(with = "serde_with::rust::StringWithSeparator::<CommaSeparator>")]
        s: Vec<String>,
    }
    let from = r#"[
        { "s": "A,B,C,D" },
        { "s": ",," },
        { "s": "AVeryLongString" }
    ]"#;

    let res: Vec<S> = serde_json::from_str(from).unwrap();
    assert_eq!(
        vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ],
        res[0].s
    );
    assert_eq!(
        vec!["".to_string(), "".to_string(), "".to_string()],
        res[1].s
    );
    assert_eq!(vec!["AVeryLongString".to_string()], res[2].s);
}

#[test]
fn string_collection_non_existing() {
    #[derive(Debug, Deserialize, Serialize)]
    struct S {
        #[serde(with = "serde_with::rust::StringWithSeparator::<CommaSeparator>")]
        s: Vec<String>,
    }
    let from = r#"[
        { "s": "" }
    ]"#;

    let res: Vec<S> = serde_json::from_str(from).unwrap();
    assert_eq!(Vec::<String>::new(), res[0].s);

    assert_eq!(r#"{"s":""}"#, serde_json::to_string(&res[0]).unwrap());
}
