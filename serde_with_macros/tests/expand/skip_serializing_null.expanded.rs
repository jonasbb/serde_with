use serde_with_macros::skip_serializing_none;
fn never<T>(_t: &T) -> bool {
    false
}
/// Test different ways to write [`Option`] and the `serialize_always` attribute.
struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: ::std::option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: std::option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    c: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    d: core::option::Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e: Option<u8>,
    f: Option<u8>,
}
/// Test how [`skip_serializing_none`][] works with existing annotations.
struct DataExistingAnnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "abc")]
    b: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<String>,
    #[serde(skip_serializing_if = "never")]
    #[serde(rename = "name")]
    d: Option<String>,
}
