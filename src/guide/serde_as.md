# `serde_as` Annotation

This is an alternative to serde's with-annotation.
It is more flexable and composable but work with fewer types.

The scheme is based on two new traits, [`SerializeAs`] and [`DeserializeAs`], which need to be implemented by all types which want to be compatible with `serde_as`.
The proc macro attribute [`#[serde_as]`][crate::serde_as] exists as a usability boost for users.

This site contains some general advice how to use this crate and then lists the implemented conversions for `serde_as`.
The basic design of the system was done by [@markazmierczak](https://github.com/markazmierczak).

1. [Switching from serde's with to `serde_as`](#switching-from-serdes-with-to-serde_as)
    1. [Implementing `SerializeAs` / `DeserializeAs`](#implementing-serializeas--deserializeas)
2. [De/Serialize Implementations Available](#deserialize-implementations-available)
    1. [Bytes / `Vec<u8>` to hex string](#bytes--vecu8-to-hex-string)
    2. [De/Serialize with `FromStr` and `Display`](#deserialize-with-fromstr-and-display)
    3. [`Duration` as seconds](#duration-as-seconds)
    4. [Ignore deserialization errors](#ignore-deserialization-errors)
    5. [`Maps` to `Vec` of tuples](#maps-to-vec-of-tuples)
    6. [`NaiveDateTime` like UTC timestamp](#naivedatetime-like-utc-timestamp)
    7. [`None` as empty `String`](#none-as-empty-string)
    8. [Value into JSON String](#value-into-json-string)
    9. [`Vec` of tuples to `Maps`](#vec-of-tuples-to-maps)

## Switching from serde's with to `serde_as`

For the user the main difference is that instead of

```rust,ignore
#[serde(with = "...")]
```

you now have to write

```rust,ignore
#[serde_as(as = "...")]
```

and place the `#[serde_as]` attribute *before* the `#[derive]` attribute.
You still need the `#[derive(Serialize, Deserialize)]` on the struct/enum.

All together this looks like:

```rust
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize)]
struct A {
    #[serde_as(as = "DisplayFromStr")]
    mime: mime::Mime,
}
```

The main advantage is that you can compose `serde_as` stuff, which is not possible with the with-annotation.
For example, the `mime` field from above could be nested in one or more data structures:

```rust
# use std::collections::BTreeMap;
# use serde::{Deserialize, Serialize};
# use serde_with::{serde_as, DisplayFromStr};
#
#[serde_as]
#[derive(Serialize, Deserialize)]
struct A {
    #[serde_as(as = "Option<BTreeMap<_, Vec<DisplayFromStr>>>")]
    mime: Option<BTreeMap<String, Vec<mime::Mime>>>,
}
```

### Implementing `SerializeAs` / `DeserializeAs`

You can support [`SerializeAs`] / [`DeserializeAs`] on your own types too.
Most "leaf" types do not need to implement these traits since they are supported implicitly.
"Leaf" type refers to types which directly serialize like plain data types.
[`SerializeAs`] / [`DeserializeAs`] is very important for collection types, like `Vec` or `BTreeMap`, since they need special handling for they key/value de/serialization such that the conversions can be done on the key/values.
You also find them implemented on the conversion types, such as the [`DisplayFromStr`] type.
These make up the bulk of this crate and allow you to perform all the nice conversions to [hex strings], the [bytes to string converter], or [duration to UNIX epoch].

## De/Serialize Implementations Available

### Bytes / `Vec<u8>` to hex string

[`Hex`]

Requires the `hex` feature.

```ignore
// Rust
#[serde_as(as = "serde_with::hex::Hex")]
value: Vec<u8>,

// JSON
"value": "deadbeef",
```

### De/Serialize with `FromStr` and `Display`

Useful if a type implements `FromStr` / `Display` but not `Deserialize` / `Serialize`.

[`DisplayFromStr`]

```ignore
// Rust
#[serde_as(as = "serde_with::DisplayFromStr")]
value: u128,
#[serde_as(as = "serde_with::DisplayFromStr")]
mime: mime::Mime,

// JSON
"value": "340282366920938463463374607431768211455",
"mime": "text/*",
```

### `Duration` as seconds

[`DurationSeconds`]

```ignore
// Rust
#[serde_as(as = "serde_with::DurationSeconds<u64>")]
value: Duration,

// JSON
"value": 86400,
```

[`DurationSecondsWithFrac`] supports subsecond precision:

```ignore
// Rust
#[serde_as(as = "serde_with::DurationSecondsWithFrac<f64>")]
value: Duration,

// JSON
"value": 1.234,
```

The same conversions are also implemented for [`chrono::Duration`] with the `chrono` feature.

### Ignore deserialization errors

Check the documentation for [`DefaultOnError`].

### `Maps` to `Vec` of tuples

```ignore
// Rust
#[serde_as(as = "Vec<(_, _)>")]
value: HashMap<String, u32>, // also works with BTreeMap

// JSON
"value": [
    ["hello", 1],
    ["world", 2]
],
```

The [inverse operation](#vec-of-tuples-to-maps) is also available.

### `NaiveDateTime` like UTC timestamp

Requires the `chrono` feature.

```ignore
// Rust
#[serde_as(as = "chrono::DateTime<chrono::Utc>")]
value: chrono::NaiveDateTime,

// JSON
"value": "1994-11-05T08:15:30Z",
                             ^ Pretend DateTime is UTC
```

### `None` as empty `String`

[`NoneAsEmptyString`]

```ignore
// Rust
#[serde_as(as = "serde_with::NoneAsEmptyString")]
value: Option<String>,

// JSON
"value": "", // converts to None

"value": "Hello World!", // converts to Some
```

### Value into JSON String

Some JSON APIs are weird and return a JSON encoded string in a JSON response

[`JsonString`]

Requires the `json` feature.

```ignore
// Rust
#[derive(Deserialize, Serialize)]
struct OtherStruct {
    value: usize,
}

#[serde_as(as = "serde_with::json::JsonString")]
value: OtherStruct,

// JSON
"value": "{\"value\":5}",
```

### `Vec` of tuples to `Maps`

```ignore
// Rust
#[serde_as(as = "HashMap<_, _>")] // also works with BTreeMap
value: Vec<String, u32>,

// JSON
"value": {
    "hello": 1,
    "world": 2
},
```

The [inverse operation](#maps-to-vec-of-tuples) is also available.

[`chrono::Duration`]: chrono::Duration
[`DefaultOnError`]: crate::DefaultOnError
[`DeserializeAs`]: crate::DeserializeAs
[`DisplayFromStr`]: crate::DisplayFromStr
[`DurationSeconds`]: crate::DurationSeconds
[`DurationSecondsWithFrac`]: crate::DurationSecondsWithFrac
[`Hex`]: crate::hex::Hex
[`JsonString`]: crate::json::JsonString
[`SerializeAs`]: crate::SerializeAs
[bytes to string converter]: crate::BytesOrString
[duration to UNIX epoch]: crate::DurationSeconds
[hex strings]: crate::hex::Hex
[`NoneAsEmptyString`]: crate::NoneAsEmptyString
