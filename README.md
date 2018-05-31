# Custom de/serialization functions for Rust's [serde](https://serde.rs)

This crate provides custom de/serialization helpers to use in combination with [serde's with-annotation][with-annotation].

## Usage

```toml
[dependencies.serde_with]
version = "..."
features = [ ... ]
```

The crate is divided into different modules.
They contain helpers for external crates and must be enabled with the correspondig feature.

Annotate your struct or enum to enable the custom de/serializer.

```rust
#[derive(Deserialize, Serialize)]
struct Foo {
    #[serde(with = "serde_with::rust::display_fromstr")]
    bar: u8,
}
```

Most helpers implement both deserialize and serialize.
If you do not want to derive both, you can simply derive only the necessary parts.
If you want to mix different helpers, you can write your annotations like

```rust
#[derive(Deserialize, Serialize)]
struct Foo {
    #[serde(
        deserialize_with = "serde_with::rust::display_fromstr::deserialize",
        serialize_with = "serde_with::json::nested::serialize"
    )]
    bar: u8,
}
```

However, this will prohibit you from applying deserialize on the value returned by serializing a struct.

[with-annotation]: https://serde.rs/field-attrs.html#serdewith--module

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
