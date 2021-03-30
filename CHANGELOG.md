# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.8.0] - 2021-03-30

### Added

* Added `PickFirst` adapter for `serde_as`. [#291]
    It allows to deserialize from multiple different forms.
    Deserializing a number from either a number or string can be implemented like:

    ```rust
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    value: u32,
    ```

* Implement `SerializeAs`/`DeserializeAs` for more wrapper types. [#288], [#293]
    This now supports:
    * `Arc`, `sync::Weak`
    * `Rc`, `rc::Weak`
    * `Cell`, `RefCell`
    * `Mutex`, `RwLock`
    * `Result`

[#288]: https://github.com/jonasbb/serde_with/issues/288
[#291]: https://github.com/jonasbb/serde_with/issues/291
[#293]: https://github.com/jonasbb/serde_with/issues/293

### Changed

* Add a new `serde_with::rust::map_as_tuple_list` module as a replacement for `serde_with::rust::btreemap_as_tuple_list` and `serde_with::rust::hashmap_as_tuple_list`.
    The new module uses `IntoIterator` and `FromIterator` as trait bound making it usable in more sitations.
    The old names continue to exist but are marked as deprecated.

### Deprecated

* Deprecated the module names `serde_with::rust::btreemap_as_tuple_list` and `serde_with::rust::hashmap_as_tuple_list`.
    You can use `serde_with::rust::map_as_tuple_list` as a replacement.

### Fixed

* Implement `Timestamp*Seconds` and `Duration*Seconds` also for chrono types.
    This closes [#194]. This was incompletely implemented in [#199].

[#194]: https://github.com/jonasbb/serde_with/issues/194
[#199]: https://github.com/jonasbb/serde_with/issues/199

## [1.7.0] - 2021-03-24

### Added

* Add support for arrays of arbitrary size. ([#272])
    This feature requires Rust 1.51+.

    ```rust
    // Rust
    #[serde_as(as = "[[_; 64]; 33]")]
    value: [[u8; 64]; 33],

    // JSON
    "value": [[0,0,0,0,0,...], [0,0,0,...], ...],
    ```

    Mapping of arrays was available before, but limited to arrays of length 32.
    All conversion methods are available for the array elements.

    This is similar to the existing [`serde-big-array`] crate with three important improvements:

    1. Support for the `serde_as` annotation.
    2. Supports non-copy elements (see [serde-big-array#6][serde-big-array-copy]).
    3. Supports arbitrary nestings of arrays (see [serde-big-array#7][serde-big-array-nested]).

[#272]: https://github.com/jonasbb/serde_with/pull/272
[`serde-big-array`]: https://crates.io/crates/serde-big-array
[serde-big-array-copy]: https://github.com/est31/serde-big-array/issues/6
[serde-big-array-nested]: https://github.com/est31/serde-big-array/issues/7

* Arrays with tuple elements can now be deserialized from  a map. ([#272])
    This feature requires Rust 1.51+.

    ```rust
    // Rust
    #[serde_as(as = "BTreeMap<_, _>")]
    value: [(String, u16); 3],

    // JSON
    "value": {
        "a": 1,
        "b": 2,
        "c": 3
    },
    ```

* The `Bytes` type is heavily inspired by `serde_bytes` and ports it to the `serde_as` system. ([#277])

    ```rust
    #[serde_as(as = "Bytes")]
    value: Vec<u8>,
    ```

    Compared to `serde_bytes` these improvements are available

    1. Integration with the `serde_as` annotation (see [serde-bytes#14][serde-bytes-complex]).
    2. Implementation for arrays of arbitrary size (Rust 1.51+) (see [serde-bytes#26][serde-bytes-arrays]).

[#277]: https://github.com/jonasbb/serde_with/pull/277
[serde-bytes-complex]: https://github.com/serde-rs/bytes/issues/14
[serde-bytes-arrays]: https://github.com/serde-rs/bytes/issues/26

* The `OneOrMany` allows to deserialize a `Vec` from either a single element or a sequence. ([#281])

    ```rust
    #[serde_as(as = "OneOrMany<_>")]
    cities: Vec<String>,
    ```

    This allows to deserialize from either `cities: "Berlin"` or `cities: ["Berlin", "Paris"]`.
    The serialization can be configured to always emit a list with `PreferMany` or emit a single element with `PreferOne`.

[#281]: https://github.com/jonasbb/serde_with/pull/281

## [1.6.4] - 2021-02-16

### Fixed

* Fix compiling when having a struct field without the `serde_as` annotation by updating `serde_with_macros`.
    This broke in 1.4.0 of `serde_with_macros`. [#267](https://github.com/jonasbb/serde_with/issues/267)

## [1.6.3] - 2021-02-15

### Changed

* Bump macro crate dependency (`serde_with_macros`) to 1.4.0 to pull in those improvements.

## [1.6.2] - 2021-01-30

### Added

* New function `serde_with::rust::deserialize_ignore_any`.
    This function allows deserializing any data and returns the default value of the type.
    This can be used in conjunction with `#[serde(other)]` to allow deserialization of unknown data carrying enum variants.

    Thanks to @lovasoa for suggesting and implementing it.

## [1.6.1] - 2021-01-24

### Added

* Add new types similar to `DurationSeconds` and `TimestampSeconds` but for base units of milliseconds, microseconds, and nanoseconds.
    The `*WithFrac` variants also exist.
* Add `SerializeAs` implementation for references.

### Changed

* Release `Sized` trait bound from `As`, `Same`, `SerializeAs`, and `SerializeAsWrap`.
    Only the serialize part is relaxed.

## [1.6.0] - 2020-11-22

### Added

* Add `DefaultOnNull` as the equivalent for `rust::default_on_null` but for the `serde_as` system.
* Support specifying a path to the `serde_with` crate for the `serde_as` and derive macros.
    This is useful when using crate renaming in Cargo.toml or while re-exporting the macros.

    Many thanks to @tobz1000 for raising the issue and contributing fixes.

### Changed

* Bump minimum supported rust version to 1.40.0

## [1.5.1] - 2020-10-07

### Fixed

* Depend on serde with the `derive` feature enabled.
    The `derive` feature is required to deserliaze untagged enums which are used in the `DefaultOnError` helpers.
    This fixes compilation of `serde_with` in scenarios where no other crate enables the `derive` feature.

## [1.5.0] - 2020-10-01

### Added

* The largest addition to this release is the addition of the `serde_as` de/serialization scheme.
    It's goal is it to be a more flexible replacement to serde's with-annotation, by being more composable than before.
    No longer is it a problem to add a custom de/serialization adapter is the type is within an `Option` or a `Vec`.

    Thanks to `@markazmierczak` for the design of the trait without whom this wouldn't be possible.

    More details about this new scheme can be found in the also new [user guide](https://docs.rs/serde_with/1.5.0/serde_with/guide/index.html)
* This release also features a detailed user guide.
    The guide focusses more on how to use this crate by providing examples.
    For example, it includes a section about the available feature flags of this crate and how you can migrate to the shiny new `serde_as` scheme.
* The crate now features de/serialization adaptors for the std and chrono's `Duration` types. #56 #104
* Add a `hex` module, which allows formatting bytes (i.e. `Vec<u8>`) as a hexadecimal string.
    The formatting supports different arguments how the formatting is happening.
* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

    This is part of `serde_with_macros` v1.2.0.
* Added some `serialize` functions to modules which previously had none.
    This makes it easier to use the conversion when also deriving `Serialialize`.
    The functions simply pass through to the underlying `Serialize` implementation.
    This affects `sets_duplicate_value_is_error`, `maps_duplicate_key_is_error`, `maps_first_key_wins`, `default_on_error`, and `default_on_null`.
* Added `sets_last_value_wins` as a replacement for `sets_first_value_wins` which is deprecated now.
    The default behavior of serde is to prefer the first value of a set so the opposite is taking the last value.
* Added `#[serde_as]` compatible conversion methods for serializing durations and timestamps as numbers.
    The four types `DurationSeconds`, `DurationSecondsWithFrac`, `TimestampSeconds`, `TimestampSecondsWithFrac` provide the serialization conversion with optional subsecond precision.
    There is support for `std::time::Duration`, `chrono::Duration`, `std::time::SystemTime` and `chrono::DateTime`.
    Timestamps are serialized as a duration since the UNIX epoch.
    The serialization can be customized.
    It supports multiple formats, such as `i64`, `f64`, or `String`, and the deserialization can be tweaked if it should be strict or lenient when accepting formats.

### Changed

* Convert the code to use 2018 edition.
* @peterjoel improved the performance of `with_prefix!`. #101

### Fixed

* The `with_prefix!` macro, to add a string prefixes during serialization, now also works with unit variant enum types. #115 #116
* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.
    This is part of `serde_with_macros` v1.2.0.

### Deprecated

* Deprecate `sets_first_value_wins`.
    The default behavior of serde is to take the first value, so this module is not necessary.

## [1.5.0-alpha.2] - 2020-08-16

### Added

* Add a `hex` module, which allows formatting bytes (i.e. `Vec<u8>`) as a hexadecimal string.
    The formatting supports different arguments how the formatting is happening.
* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

    This is part of `serde_with_macros` v1.2.0-alpha.3.

### Fixed

* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.
    This is part of `serde_with_macros` v1.2.0-alpha.2.

## [1.5.0-alpha.1] - 2020-06-27

### Added

* The largest addition to this release is the addition of the `serde_as` de/serialization scheme.
    It's goal is it to be a more flexible replacement to serde's with-annotation, by being more composable than before.
    No longer is it a problem to add a custom de/serialization adapter is the type is within an `Option` or a `Vec`.

    Thanks to `@markazmierczak` for the design of the trait without whom this wouldn't be possible.

    More details about this new scheme can be found in the also new [user guide](https://docs.rs/serde_with/1.5.0-alpha.1/serde_with/guide/index.html)
* This release also features a detailed user guide.
    The guide focusses more on how to use this crate by providing examples.
    For example, it includes a section about the available feature flags of this crate and how you can migrate to the shiny new `serde_as` scheme.
* The crate now features de/serialization adaptors for the std and chrono's `Duration` types. #56 #104

### Changed

* Convert the code to use 2018 edition.
* @peterjoel improved the performance of `with_prefix!`. #101

### Fixed

* The `with_prefix!` macro, to add a string prefixes during serialization, now also works with unit variant enum types. #115 #116

## [1.4.0] - 2020-01-16

### Added

* Add a helper to deserialize a `Vec<u8>` from `String` (#35)
* Add `default_on_error` helper, which turns errors into `Default`s of the type
* Add `default_on_null` helper, which turns `null` values into `Default`s of the type

### Changed

* Bump minimal Rust version to 1.36.0
    * Supports Rust Edition 2018
    * version-sync depends on smallvec which requires 1.36
* Improved CI pipeline by running `cargo audit` and `tarpaulin` in all configurations now.

## [1.3.1] - 2019-04-09

### Fixed

* Use `serde_with_macros` with proper dependencies specified.

## [1.3.0] - 2019-04-02

### Added

* Add `skip_serializing_none` attribute, which adds `#[serde(skip_serializing_if = "Option::is_none")]` for each Option in a struct.
    This is helpfull for APIs which have many optional fields.
    The effect of can be negated by adding `serialize_always` on those fields, which should always be serialized.
    Existing `skip_serializing_if` will never be modified and those fields keep their behavior.

## [1.2.0] - 2019-03-04

### Added

* Add macro helper to support deserializing values with nested or flattened syntax #38
* Serialize tuple list as map helper

### Changed

* Bumped minimal Rust version to 1.30.0

## [1.1.0] - 2019-02-18

### Added

* Serialize HashMap/BTreeMap as list of tuples

## [1.0.0] - 2019-01-17

### Added

* No changes in this release.
* Bumped version number to indicate the stability of the library.

## [0.2.5] - 2018-11-29

### Added

* Helper which deserializes an empty string as `None` and otherwise uses `FromStr` and `AsRef<str>`.

## [0.2.4] - 2018-11-24

### Added

* De/Serialize sequences by using `Display` and `FromStr` implementations on each element. Contributed by @katyo

## [0.2.3] - 2018-11-08

### Added

* Add missing docs and enable deny missing_docs
* Add badges to Cargo.toml and crates.io

### Changed

* Improve Travis configuration
* Various clippy improvements

## [0.2.2] - 2018-08-05

### Added

* `unwrap_or_skip` allows to transparently serialize the inner part of a `Some(T)`
* Add deserialization helpser for sets and maps, inspired by [comment](https://github.com/serde-rs/serde/issues/553#issuecomment-299711855)
    * Create an error if duplicate values for a set are detected
    * Create an error if duplicate keys for a map are detected
    * Implement a first-value wins strategy for sets/maps. This is different to serde's default
        which implements a last value wins strategy.

## [0.2.1] - 2018-06-05

### Added

* Double Option pattern to differentiate between missing, unset, or existing value
* `with_prefix!` macro, which puts a prefix on every struct field

## [0.2.0] - 2018-05-31

### Added

* Add chrono support: Deserialize timestamps from int, float, and string
* Serialization of embedded JSON strings
* De/Serialization using `Display` and `FromStr` implementations
* String-based collections using `Display` and `FromStr`, allows to deserialize "#foo,#bar"

## [0.1.0] - 2017-08-17

### Added

* Reserve name on crates.io
