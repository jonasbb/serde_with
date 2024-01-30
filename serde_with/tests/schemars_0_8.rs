use crate::utils::{check_matches_schema, check_valid_json_schema};
use ::schemars_0_8::JsonSchema;
use expect_test::expect_file;
use serde::Serialize;
use serde_json::json;
use serde_with::*;
use std::collections::BTreeSet;

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
            #[serde_as]
            #[derive(JsonSchema, Serialize)]
            $( #[$stattr] )*
            struct $name {
                $(
                    $( #[$fattr] )*
                    $field: $ty,
                )*
            }

            let schema = schemars::schema_for!($name);
            let mut schema = serde_json::to_string_pretty(&schema)
                .expect("schema could not be serialized");
            schema.push('\n');

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

    #[serde_as]
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
    let mut schema = serde_json::to_string_pretty(&schema).expect("schema could not be serialized");
    schema.push('\n');

    let expected = expect_file!["./schemars_0_8/schemars_basic.json"];
    expected.assert_eq(&schema);
}

#[test]
fn schemars_custom_with() {
    #[serde_as]
    #[derive(JsonSchema, Serialize)]
    struct Test {
        #[serde_as(as = "DisplayFromStr")]
        #[schemars(with = "i32")]
        custom: i32,

        #[serde_as(as = "DisplayFromStr")]
        #[cfg_attr(any(), schemars(with = "i32"))]
        with_disabled: i32,

        #[serde_as(as = "DisplayFromStr")]
        #[cfg_attr(all(), schemars(with = "i32"))]
        always_enabled: i32,
    }

    check_matches_schema::<Test>(&json!({
        "custom": 3,
        "with_disabled": "5",
        "always_enabled": 7,
    }));
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
    use serde_with::formats::*;
    use std::collections::BTreeSet;

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

        set_last_value_wins {
            struct Test {
                #[serde_as(as = "SetLastValueWins<_>")]
                data: BTreeSet<u32>,
            }
        }

        set_prevent_duplicates {
            struct Test {
                #[serde_as(as = "SetPreventDuplicates<_>")]
                data: BTreeSet<u32>,
            }
        }

        duration {
            struct Test {
                #[serde_as(as = "DurationSeconds<u64, Flexible>")]
                seconds: std::time::Duration,

                #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
                frac: std::time::Duration,

                #[serde_as(as = "DurationSeconds<String, Flexible>")]
                flexible_string: std::time::Duration,

                #[serde_as(as = "DurationSeconds<u64, Strict>")]
                seconds_u64_strict: std::time::Duration,

                #[serde_as(as = "TimestampSeconds<i64, Flexible>")]
                time_i64: std::time::SystemTime,
            }
        }
    }
}

mod derive {
    use super::*;

    #[serde_as]
    #[derive(Serialize)]
    #[cfg_attr(all(), derive(JsonSchema))]
    struct Enabled {
        #[serde_as(as = "DisplayFromStr")]
        field: u32,
    }

    #[serde_as]
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

    #[serde_as]
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

mod bool_from_int {
    use super::*;
    use serde_with::formats::{Flexible, Strict};

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct BoolStrict {
        #[serde_as(as = "BoolFromInt<Strict>")]
        value: bool,
    }

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct BoolFlexible {
        #[serde_as(as = "BoolFromInt<Flexible>")]
        value: bool,
    }

    #[test]
    fn test_serialized_strict_is_valid() {
        check_valid_json_schema(&vec![
            BoolStrict { value: true },
            BoolStrict { value: false },
        ]);
    }

    #[test]
    fn test_serialized_flexible_is_valid() {
        check_valid_json_schema(&vec![
            BoolFlexible { value: true },
            BoolFlexible { value: false },
        ]);
    }

    #[test]
    #[should_panic]
    fn strict_out_of_range() {
        check_matches_schema::<BoolStrict>(&json!({
            "value": 5
        }));
    }

    #[test]
    fn flexible_out_of_range() {
        check_matches_schema::<BoolFlexible>(&json!({
            "value": 5
        }));
    }

    #[test]
    #[should_panic]
    fn flexible_wrong_type() {
        check_matches_schema::<BoolFlexible>(&json!({
            "value": "seven"
        }));
    }

    #[test]
    #[should_panic]
    fn test_fractional_value_strict() {
        check_matches_schema::<BoolStrict>(&json!({
            "value": 0.5
        }))
    }

    #[test]
    #[should_panic]
    fn test_fractional_value_flexible() {
        check_matches_schema::<BoolFlexible>(&json!({
            "value": 0.5
        }))
    }
}

mod bytes_or_string {
    use super::*;

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct Test {
        #[serde_as(as = "BytesOrString")]
        bytes: Vec<u8>,
    }

    #[test]
    fn test_serialized_is_valid() {
        check_valid_json_schema(&Test {
            bytes: b"test".to_vec(),
        });
    }

    #[test]
    fn test_string_valid_json() {
        check_matches_schema::<Test>(&json!({
            "bytes": "test string"
        }));
    }

    #[test]
    fn test_bytes_valid_json() {
        check_matches_schema::<Test>(&json!({
            "bytes": [1, 2, 3, 4]
        }));
    }

    #[test]
    #[should_panic]
    fn test_int_not_valid_json() {
        check_matches_schema::<Test>(&json!({
            "bytes": 5
        }));
    }
}

mod duration {
    use super::*;
    use serde_with::formats::{Flexible, Strict};
    use std::time::{Duration, SystemTime};

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct DurationTest {
        #[serde_as(as = "DurationSeconds<u64, Strict>")]
        strict_u64: Duration,

        #[serde_as(as = "DurationSeconds<String, Strict>")]
        strict_str: Duration,

        #[serde_as(as = "DurationSecondsWithFrac<f64, Strict>")]
        strict_f64: Duration,

        #[serde_as(as = "DurationSeconds<u64, Flexible>")]
        flexible_u64: Duration,

        #[serde_as(as = "DurationSeconds<f64, Flexible>")]
        flexible_f64: Duration,

        #[serde_as(as = "DurationSeconds<String, Flexible>")]
        flexible_str: Duration,
    }

    #[test]
    fn test_serialized_is_valid() {
        check_valid_json_schema(&DurationTest {
            strict_u64: Duration::from_millis(2500),
            strict_str: Duration::from_millis(2500),
            strict_f64: Duration::from_millis(2500),
            flexible_u64: Duration::from_millis(2500),
            flexible_f64: Duration::from_millis(2500),
            flexible_str: Duration::from_millis(2500),
        });
    }

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct FlexibleU64Duration(#[serde_as(as = "DurationSeconds<u64, Flexible>")] Duration);

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct FlexibleStringDuration(#[serde_as(as = "DurationSeconds<String, Flexible>")] Duration);

    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct FlexibleTimestamp(#[serde_as(as = "TimestampSeconds<i64, Flexible>")] SystemTime);

    #[test]
    fn test_string_as_flexible_u64() {
        check_matches_schema::<FlexibleU64Duration>(&json!("32"));
    }

    #[test]
    fn test_integer_as_flexible_u64() {
        check_matches_schema::<FlexibleU64Duration>(&json!(16));
    }

    #[test]
    fn test_number_as_flexible_u64() {
        check_matches_schema::<FlexibleU64Duration>(&json!(54.1));
    }

    #[test]
    #[should_panic]
    fn test_negative_as_flexible_u64() {
        check_matches_schema::<FlexibleU64Duration>(&json!(-5));
    }

    #[test]
    fn test_string_as_flexible_string() {
        check_matches_schema::<FlexibleStringDuration>(&json!("32"));
    }

    #[test]
    fn test_integer_as_flexible_string() {
        check_matches_schema::<FlexibleStringDuration>(&json!(16));
    }

    #[test]
    fn test_number_as_flexible_string() {
        check_matches_schema::<FlexibleStringDuration>(&json!(54.1));
    }

    #[test]
    #[should_panic]
    fn test_negative_as_flexible_string() {
        check_matches_schema::<FlexibleStringDuration>(&json!(-5));
    }

    #[test]
    fn test_negative_as_flexible_timestamp() {
        check_matches_schema::<FlexibleTimestamp>(&json!(-50000));
    }

    #[test]
    fn test_negative_string_as_flexible_timestamp() {
        check_matches_schema::<FlexibleTimestamp>(&json!("-50000"));
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

#[test]
fn test_set_last_value_wins_with_duplicates() {
    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct Test {
        #[serde_as(as = "SetLastValueWins<_>")]
        set: BTreeSet<u32>,
    }

    check_matches_schema::<Test>(&json!({
        "set": [ 1, 2, 3, 1, 4, 2 ]
    }));
}

#[test]
#[should_panic]
fn test_set_prevent_duplicates_with_duplicates() {
    #[serde_as]
    #[derive(Serialize, JsonSchema)]
    struct Test {
        #[serde_as(as = "SetPreventDuplicates<_>")]
        set: BTreeSet<u32>,
    }

    check_matches_schema::<Test>(&json!({
        "set": [ 1, 1 ]
    }));
}
