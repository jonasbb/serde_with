use serde_with_macros::skip_serializing_none;

fn never<T>(_t: &T) -> bool {
    false
}

/// Test different ways to write [`Option`] and the `serialize_always` attribute.
#[skip_serializing_none]
struct Data {
    a: ::std::option::Option<String>,
    b: std::option::Option<String>,
    c: ::std::option::Option<i64>,
    d: core::option::Option<i64>,

    e: Option<u8>,
    #[serialize_always]
    f: Option<u8>,
}

/// Test how [`skip_serializing_none`][] works with existing annotations.
#[skip_serializing_none]
struct DataExistingAnnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "abc")]
    b: Option<String>,
    #[serde(default)]
    c: Option<String>,
    #[serde(skip_serializing_if = "never")]
    #[serde(rename = "name")]
    d: Option<String>,
}
