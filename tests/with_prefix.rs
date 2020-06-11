use pretty_assertions::assert_eq;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use serde_with::with_prefix;
use std::collections::HashMap;

#[test]
fn test_flatten_with_prefix() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Match {
        #[serde(flatten, with = "prefix_player1")]
        player1: Player,
        #[serde(flatten, with = "prefix_player2")]
        player2: Option<Player>,
        #[serde(flatten, with = "prefix_player3")]
        player3: Option<Player>,
        #[serde(flatten, with = "prefix_tag")]
        tags: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Player {
        name: String,
        votes: u64,
    }

    with_prefix!(prefix_player1 "player1_");
    with_prefix!(prefix_player2 "player2_");
    with_prefix!(prefix_player3 "player3_");
    with_prefix!(prefix_tag "tag_");

    let m = Match {
        player1: Player {
            name: "name1".to_owned(),
            votes: 1,
        },
        player2: Some(Player {
            name: "name2".to_owned(),
            votes: 2,
        }),
        player3: None,
        tags: {
            let mut tags = HashMap::new();
            tags.insert("t".to_owned(), "T".to_owned());
            tags
        },
    };

    let expected = json!({
        "player1_name": "name1",
        "player1_votes": 1,
        "player2_name": "name2",
        "player2_votes": 2,
        "tag_t": "T"
    });

    let j = serde_json::to_string(&m).unwrap();
    assert_eq!(j, expected.to_string());

    assert_eq!(m, serde_json::from_str(&j).unwrap());
}

#[test]
fn test_plain_with_prefix() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Match {
        #[serde(with = "prefix_player1")]
        player1: Player,
        #[serde(with = "prefix_player2")]
        player2: Option<Player>,
        #[serde(with = "prefix_player3")]
        player3: Option<Player>,
        #[serde(with = "prefix_tag")]
        tags: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Player {
        name: String,
        votes: u64,
    }

    with_prefix!(prefix_player1 "player1_");
    with_prefix!(prefix_player2 "player2_");
    with_prefix!(prefix_player3 "player3_");
    with_prefix!(prefix_tag "tag_");

    let m = Match {
        player1: Player {
            name: "name1".to_owned(),
            votes: 1,
        },
        player2: Some(Player {
            name: "name2".to_owned(),
            votes: 2,
        }),
        player3: None,
        tags: {
            let mut tags = HashMap::new();
            tags.insert("t".to_owned(), "T".to_owned());
            tags
        },
    };

    let expected = json!({
        "player1": {
            "player1_name": "name1",
            "player1_votes": 1,
        },
        "player2": {
            "player2_name": "name2",
            "player2_votes": 2,
        },
        "player3": null,
        "tags": {
            "tag_t": "T"
        }
    });

    let j = serde_json::to_string(&m).unwrap();
    assert_eq!(j, expected.to_string());

    assert_eq!(m, serde_json::from_str(&j).unwrap());
}

/// Ensure that with_prefix works for unit type enum variants.
#[test]
fn test_enum_unit_variant_with_prefix() {
    use std::collections::BTreeMap;

    #[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Ord, PartialOrd)]
    enum Foo {
        One,
        Two,
        Three,
    }

    #[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Ord, PartialOrd)]
    struct Data {
        stuff: String,

        #[serde(flatten, with = "foo")]
        foo: BTreeMap<Foo, i32>,
    }
    with_prefix!(foo "foo_");

    let mut my_foo = BTreeMap::new();
    my_foo.insert(Foo::One, 1);
    my_foo.insert(Foo::Two, 2);
    my_foo.insert(Foo::Three, 3);

    let d = Data {
        stuff: "Stuff".to_owned(),
        foo: my_foo,
    };

    let expected = json!({
        "stuff": "Stuff",
        "foo_One": 1,
        "foo_Two": 2,
        "foo_Three": 3,
    });

    let j = serde_json::to_string(&d).unwrap();
    assert_eq!(j, expected.to_string());

    assert_eq!(d, serde_json::from_str(&j).unwrap());
}
