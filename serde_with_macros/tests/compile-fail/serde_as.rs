use serde::Serialize;
use serde_with_macros::serde_as;

/// Test error message for conflicting *_as-annotations
#[serde_as]
#[derive(Serialize)]
struct ConflictingAsAnnotations {
    #[serde_as(as = "_", deserialize_as = "_")]
    a: u32,
    #[serde_as(as = "_", serialize_as = "_")]
    b: u32,
    #[serde_as(as = "_", deserialize_as = "_", serialize_as = "_")]
    c: u32,
}

/// Test error message for conflicts with serde's with-annotation
#[serde_as]
#[derive(Serialize)]
struct ConflictingWithAnnotations {
    #[serde_as(as = "_")]
    #[serde(with = "u32")]
    a: u32,
    #[serde_as(as = "_")]
    #[serde(deserialize_with = "u32")]
    b: u32,
    #[serde_as(as = "_")]
    #[serde(serialize_with = "u32")]
    c: u32,

    #[serde_as(deserialize_as = "_")]
    #[serde(with = "u32")]
    d: u32,
    #[serde_as(deserialize_as = "_")]
    #[serde(deserialize_with = "u32")]
    e: u32,
    #[serde_as(deserialize_as = "_")]
    #[serde(serialize_with = "u32")]
    f: u32,

    #[serde_as(serialize_as = "_")]
    #[serde(with = "u32")]
    g: u32,
    #[serde_as(serialize_as = "_")]
    #[serde(deserialize_with = "u32")]
    h: u32,
    #[serde_as(serialize_as = "_")]
    #[serde(serialize_with = "u32")]
    i: u32,
}

fn main() {}
