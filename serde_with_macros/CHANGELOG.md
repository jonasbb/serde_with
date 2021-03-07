# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.4.1] - 2021-02-16

### Fixed

* Fix compiling when having a struct field without the `serde_as` annotation.
    This broke in 1.4.0 [#267](https://github.com/jonasbb/serde_with/issues/267)

## [1.4.0] - 2021-02-15

### Changed

* Improve error messages when `#[serde_as(..)]` is misused as a field attribute.
    Thanks to @Lehona for reporting the bug in #233.
* Internal cleanup for assembling and parsing attributes during `serde_as` processing.
* Change processing on `#[serde_as(...)]` attributes on fields.

    The attributes will no longer be stripped during proc-macro processing.
    Instead, a private derive macro is applied to the struct/enum which captures them and makes them inert, thus allowing compilation.

    This should have no effect on the generated code and on the runtime behavior.
    It eases integration of third-party crates with `serde_with`, since they can now process the `#[serde_as(...)]` field attributes reliably.
    Before this was impossible for derive macros and lead to akward ordering constraints on the attribute macros.

    Thanks to @Lehona for reporting this problem and to @dtolnay for suggesting the dummy derive macro.

## [1.3.0] - 2020-11-22

### Added

* Support specifying a path to the `serde_with` crate for the `serde_as` and derive macros.
    This is useful when using crate renaming in Cargo.toml or while re-exporting the macros.

    Many thanks to @tobz1000 for raising the issue and contributing fixes.

### Changed

* Bump minimum supported rust version to 1.40.0

## [1.2.2] - 2020-10-06

### Fixed

* @adwhit contributed an improvement to `DeserializeFromStr` which allows it to deserialize from bytes (#186).
    This makes the derived implementation applicable in more situations.

## [1.2.1] - 2020-10-04

### Fixed

* The derive macros `SerializeDisplay` and `DeserializeFromStr` now use the properly namespaced types and traits.
    This solves conflicts with `Result` if `Result` is not `std::result::Result`, e.g., a type alias.
    Additionally, the code assumed that `FromStr` was in scope, which is now also not required.

    Thanks goes to @adwhit for reporting and fixing the problem in #186.

## [1.2.0] - 2020-10-01

### Added

* Add `serde_as` macro. Refer to the `serde_with` crate for details.
* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

### Changed

* Convert the code to use 2018 edition.

### Fixed

* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.

## [1.2.0-alpha.3] - 2020-08-16

### Added

* Add two derive macros, `SerializeDisplay` and `DeserializeFromStr`, which implement the `Serialize`/`Deserialize` traits based on `Display` and `FromStr`.
    This is in addition to the already existing methods like `DisplayFromStr`, which act locally, whereas the derive macros provide the traits expected by the rest of the ecosystem.

## [1.2.0-alpha.2] - 2020-08-08

### Fixed

* The `serde_as` macro now supports serde attributes and no longer panic on unrecognized values in the attribute.

## [1.2.0-alpha.1] - 2020-06-27

### Added

* Add `serde_as` macro. Refer to the `serde_with` crate for details.

### Changed

* Convert the code to use 2018 edition.

## [1.1.0] - 2020-01-16

### Changed

* Bump minimal Rust version to 1.36.0 to support Rust Edition 2018
* Improved CI pipeline by running `cargo audit` and `tarpaulin` in all configurations now.

## [1.0.1] - 2019-04-09

### Fixed

* Features for the `syn` dependency were missing.
    This was hidden due to the dev-dependencies whose features leaked into the normal build.

## [1.0.0] - 2019-04-02

Initial Release

### Added

* Add `skip_serializing_none` attribute, which adds `#[serde(skip_serializing_if = "Option::is_none")]` for each Option in a struct.
    This is helpfull for APIs which have many optional fields.
    The effect of can be negated by adding `serialize_always` on those fields, which should always be serialized.
    Existing `skip_serializing_if` will never be modified and those fields keep their behavior.
