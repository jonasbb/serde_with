# `serde_with` User Guide

This crate provides helper functions to extend and change how [`serde`] serializes different datatypes.
For example, you can serialize [a map as a sequence of tuples][btreemap_as_tuple_list], serialize [using the `Display` and `FromStr` traits][display_fromstr], or serialize [an empty `String` like `None`][string_empty_as_none].
`serde_with` covers types from the Rust Standard Library and some common crates like [`chrono`][serde_with_chrono].

The crate offers three types of functionality.

## 1. Integration with serde's with-annotation

[serde's with-annotation][with-annotation] allows to specify a different serialization or deserialization function for a field.
It is usefull to adapt the serialization of existing types to the requirements of a protocol.
Most modules in this crate can be used together with the with-annotation.

The annotation approach has one big drawback, in that it is very inflexible.
It allows to specify arbitrary serialization code, but the code has to perform the correct transformations.
It is not possible to combine multiple of those functions.
One common use case for this is the serialization of collections like `Vec`.
If you have a field of type `T`, you can apply the with-annotation, but if you have a field of type `Vec<T>`, there is no way to re-use the same functions for the with-annotation.
This inflexibility is fixed with the `serde_as` scheme below.

### Example

```rust
# use serde_derive::{Deserialize, Serialize};
# use std::net::Ipv4Addr;
#
# #[derive(Debug, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct Data {
    // Type does not implement Serialize or Deserialize
    #[serde(with = "serde_with::rust::display_fromstr")]
    address: Ipv4Addr,
    // Treat the Vec like a map with duplicates
    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    vec_as_map: Vec<(String, u32)>,
}

let data = Data {
    address: Ipv4Addr::new(192, 168, 0, 1),
    vec_as_map: vec![
        ("Hello".into(), 123),
        ("World".into(), 456),
        ("Hello".into(), 123),
    ],
};

let json = r#"{
  "address": "192.168.0.1",
  "vec_as_map": {
    "Hello": 123,
    "World": 456,
    "Hello": 123
  }
}"#;

// Test Serialization
assert_eq!(json, serde_json::to_string_pretty(&data).unwrap());
// Test Deserialization
assert_eq!(data, serde_json::from_str(json).unwrap());
```

## 2. A more flexible and composable replacement for the with annotation, called `serde_as` *(v1.5.0+)*

This is an alternative to the with-annotation, which adds flexibility and composability to the scheme.
The main downside is that it work with fewer types than aboves with-annotations.
However, all types from the Rust Standard Library should be supported in all combinations and any missing entry is a bug.

The `serde_as` scheme is based on two new traits: [`SerializeAs`][] and [`DeserializeAs`][].

### Example

```rust
# use serde_derive::{Deserialize, Serialize};
# use serde_with::{serde_as, DisplayFromStr};
# use std::collections::HashMap;
# use std::net::Ipv4Addr;
#
#[serde_as]
# #[derive(Debug, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct Data {
    // Type does not implement Serialize or Deserialize
    #[serde_as(as = "DisplayFromStr")]
    address: Ipv4Addr,
    // Treat the Vec like a map with duplicates
    // Convert u32 into a String and keep the String the same type
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    vec_as_map: Vec<(u32, String)>,
}

let data = Data {
    address: Ipv4Addr::new(192, 168, 0, 1),
    vec_as_map: vec![
        (123, "Hello".into()),
        (456, "World".into()),
        (123, "Hello".into()),
    ],
};

let json = r#"{
  "address": "192.168.0.1",
  "vec_as_map": {
    "123": "Hello",
    "456": "World",
    "123": "Hello"
  }
}"#;

// Test Serialization
assert_eq!(json, serde_json::to_string_pretty(&data).unwrap());
// Test Deserialization
assert_eq!(data, serde_json::from_str(json).unwrap());
```

## 3. proc-macros to make it easier to use both above parts

The proc-macros are an optional addition and improve the user exerience for common tasks.
We have already seen how the `serde_as` attribute is used to define the serialization instructions.

The proc-macro attributes are defined in the [`serde_with_macros`][] crate and re-exported from the root of this crate.
The proc-macros are optional, but enabled by default.
For futher details, please refer to the documentation of each proc-macro.

## Migrating from the with-annotations to `serde_as`

The `serde_as` scheme is the new addition to this crate and often more flexible than the with-annotations.
Information on how to migrate to the newer scheme are in the dedicated [migration guide][].

[btreemap_as_tuple_list]: crate::rust::btreemap_as_tuple_list
[display_fromstr]: crate::rust::display_fromstr
[migration guide]: crate::guide::migrating
[serde_with_chrono]: crate::chrono
[string_empty_as_none]: crate::rust::string_empty_as_none
[with-annotation]: https://serde.rs/field-attrs.html#with
[`serde_with_macros`]: serde_with_macros
