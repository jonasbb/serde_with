use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::skip_serializing_if;

mod wrapper {
    #[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Debug)]
    pub enum Wrapper {
        Nothing,
        Something(i32),
    }

    impl Wrapper {
        pub fn is_nothing(&self) -> bool {
            match self {
                Wrapper::Nothing => true,
                Wrapper::Something(_) => false,
            }
        }
    }

    impl Default for Wrapper {
        fn default() -> Self {
            Self::Nothing
        }
    }
}

#[test]
fn test_basic() {
    use serde_with_macros::skip_serializing_if;
    use wrapper::Wrapper;

    #[skip_serializing_if("Wrapper::is_nothing")]
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
    #[serde(default)]
    struct DataBasic {
        a: Wrapper,
        b: Wrapper,
        c: Wrapper,
        d: Wrapper,
    }

    let expected = json!({});
    let data = DataBasic::default();
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
    assert_eq!(data, serde_json::from_value(res).unwrap());
}

#[test]
fn test_qualified() {
    use serde_with_macros::skip_serializing_if;

    #[skip_serializing_if("wrapper::Wrapper::is_nothing")]
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
    #[serde(default)]
    struct DataBasic {
        a: wrapper::Wrapper,
        b: wrapper::Wrapper,
        c: wrapper::Wrapper,
        d: wrapper::Wrapper,
    }

    let expected = json!({});
    let data = DataBasic::default();
    let res = serde_json::to_value(&data).unwrap();
    assert_eq!(expected, res);
    assert_eq!(data, serde_json::from_value(res).unwrap());
}
