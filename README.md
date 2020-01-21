# Custom de/serialization functions for Rust's [serde](https://serde.rs)

[![docs.rs badge](https://docs.rs/serde_with/badge.svg)](https://docs.rs/serde_with/)
[![crates.io badge](https://img.shields.io/crates/v/serde_with.svg)](https://crates.io/crates/serde_with/)
[![Build Status](https://github.com/jonasbb/serde_with/workflows/Rust%20CI/badge.svg)](https://github.com/jonasbb/serde_with)
[![codecov](https://codecov.io/gh/jonasbb/serde_with/branch/master/graph/badge.svg)](https://codecov.io/gh/jonasbb/serde_with)

---

This crate provides custom de/serialization helpers to use in combination with [serde's with-annotation][with-annotation].

Serde tracks a wishlist of similar helpers at [serde#553].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.serde_with]
version = "1.4.0"
features = [ "..." ]
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

## Attributes

The crate comes with custom attributes, which futher extend how serde serialization can be customized.
They are enabled by default, but can be disabled, by removing the default features from this crate.

[with-annotation]: https://serde.rs/field-attrs.html#with
[serde#553]: https://github.com/serde-rs/serde/issues/553

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
