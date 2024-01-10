use crate::utils::{check_matches_schema, check_valid_json_schema};
use ::schemars_0_8::JsonSchema;
use expect_test::expect_file;
use serde::Serialize;
use serde_json::json;
use serde_with::*;

// This avoids us having to add `#[schemars(crate = "::schemars_0_8")]` all
// over the place. We're not testing that and it is inconvenient.
extern crate schemars_0_8 as schemars;

mod utils;

/// Declare a snapshot tests for a struct.
///
/// The snapshot files are stored under the `schemars_0_8` folder alongside
/// this test file.
macro_rules! declare_snapshot_test {
    {$(
        $( #[$tattr:meta] )*
        $test:ident {
            $( #[$stattr:meta] )*
            struct $name:ident {
                $(
                    $( #[ $fattr:meta ] )*
                    $field:ident : $ty:ty
                ),*
                $(,)?
            }
        }
    )*} => {$(
        #[test]
        $(#[$tattr])*
        fn $test() {
            #[serde_with::serde_as]
            #[derive(JsonSchema, Serialize)]
            $( #[$stattr] )*
            struct $name {
                $(
                    $( #[$fattr] )*
                    $field: $ty,
                )*
            }

            let schema = schemars::schema_for!($name);
            let schema = serde_json::to_string_pretty(&schema)
                .expect("schema could not be serialized");

            let filename = concat!("./", module_path!(), "::", stringify!($test), ".json")
                .replace("::", "/");

            let expected = expect_file![filename];
            expected.assert_eq(&schema);
        }
    )*}
}

#[test]
fn schemars_basic() {
    use ::schemars_0_8::JsonSchema;
    use serde::Serialize;

    #[serde_with::serde_as]
    #[derive(JsonSchema, Serialize)]
    #[schemars(crate = "::schemars_0_8")]
    struct Basic {
        /// Basic field, no attribute
        bare_field: u32,

        /// Field that directly uses DisplayFromStr
        #[serde_as(as = "DisplayFromStr")]
        display_from_str: u32,

        /// Same does not implement JsonSchema directly so this checks that the
        /// correct schemars attribute was injected.
        #[serde_as(as = "Same")]
        same: u32,

        /// This checks that Same still works when wrapped in a box.
        #[serde_as(as = "Box<Same>")]
        box_same: Box<u32>,

        /// Same thing, but with a Vec this time.
        #[serde_as(as = "Vec<_>")]
        vec_same: Vec<u32>,
    }

    let schema = schemars::schema_for!(Basic);
    let schema = serde_json::to_string_pretty(&schema).expect("schema could not be serialized");

    let expected = expect_file!["./schemars_0_8/schemars_basic.json"];
    expected.assert_eq(&schema);
}

mod test_std {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, VecDeque};

    declare_snapshot_test! {
        option {
            struct Test {
                #[serde_with(as = "Option<_>")]
                optional: Option<i32>,
            }
        }

        vec {
            struct Test {
                #[serde_with(as = "Vec<_>")]
                vec: Vec<String>
            }
        }

        vec_deque {
            struct Test {
                #[serde_with(as = "VecDeque<_>")]
                vec_deque: VecDeque<String>
            }
        }

        map {
            struct Test {
                #[serde_with(as = "BTreeMap<_, _>")]
                map: BTreeMap<String, i32>
            }
        }

        set {
            struct Test {
                #[serde_with(as = "BTreeSet<_>")]
                map: BTreeSet<String>,
            }
        }

        tuples {
            struct Test {
                #[serde_with(as = "()")]
                tuple0: (),

                #[serde_with(as = "(_ ,)")]
                tuple1: (i32,),

                #[serde_with(as = "(_, _)")]
                tuple2: (i32, i32),

                #[serde_with(as = "(_, _, _)")]
                tuple3: (i32, i32, String)
            }
        }
    }
}

mod snapshots {
    use super::*;
    use serde_with::formats::CommaSeparator;

    declare_snapshot_test! {
        bytes {
            struct Test {
                #[serde_as(as = "Bytes")]
                bytes: Vec<u8>,
            }
        }

        default_on_null {
            struct Test {
                #[serde_as(as = "DefaultOnNull<_>")]
                data: String,
            }
        }

        string_with_separator {
            struct Test {
                #[serde_as(as = "StringWithSeparator<CommaSeparator, String>")]
                data: Vec<String>,
            }
        }

        from_into {
            struct Test {
                #[serde_as(as = "FromInto<u64>")]
                data: u32,
            }
        }

        map {
            struct Test {
                #[serde_as(as = "Map<_, _>")]
                data: Vec<(String, u32)>,
            }
        }

        map_fixed {
            struct Test {
                #[serde_as(as = "Map<_, _>")]
                data: [(String, u32); 4],
            }
        }
    }
}

mod derive {
    use super::*;

    #[serde_with::serde_as]
    #[derive(Serialize)]
    #[cfg_attr(all(), derive(JsonSchema))]
    struct Enabled {
        #[serde_as(as = "DisplayFromStr")]
        field: u32,
    }

    #[serde_with::serde_as]
    #[derive(Serialize)]
    #[cfg_attr(any(), derive(JsonSchema))]
    struct Disabled {
        // If we are incorrectly adding `#[schemars(with = ...)]` attributes
        // then we should get an error on this field.
        #[serde_as(as = "DisplayFromStr")]
        field: u32,
    }

    #[test]
    fn test_enabled_has_correct_schema() {
        check_valid_json_schema(&Enabled { field: 77 });
    }
}

mod array {
    use super::*;

    #[serde_with::serde_as]
    #[derive(JsonSchema, Serialize)]
    struct FixedArray {
        #[serde_as(as = "[_; 3]")]
        array: [u32; 3],
    }

    #[test]
    fn test_serialized_is_valid() {
        let array = FixedArray { array: [1, 2, 3] };

        check_valid_json_schema(&array);
    }

    #[test]
    fn test_valid_json() {
        let value = json!({ "array": [1, 2, 3] });
        check_matches_schema::<FixedArray>(&value);
    }

    #[test]
    #[should_panic]
    fn test_too_short() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [1],
        }));
    }

    #[test]
    #[should_panic]
    fn test_too_long() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [1, 2, 3, 4]
        }));
    }

    #[test]
    #[should_panic]
    fn test_wrong_item_type() {
        check_matches_schema::<FixedArray>(&json!({
            "array": ["1", "2", "3"]
        }));
    }

    #[test]
    #[should_panic]
    fn test_oob_item() {
        check_matches_schema::<FixedArray>(&json!({
            "array": [-1, 0x1_0000_0000i64, 32]
        }))
    }
}

#[test]
fn test_borrow_cow() {
    use std::borrow::Cow;

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct Borrowed<'a> {
        #[serde_as(as = "BorrowCow")]
        data: Cow<'a, str>,
    }

    check_valid_json_schema(&Borrowed {
        data: Cow::Borrowed("test"),
    });
}

#[test]
fn test_map() {
    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct Test {
        map: [(&'static str, u32); 2],
    }

    check_valid_json_schema(&Test {
        map: [("a", 1), ("b", 2)],
    });
}
