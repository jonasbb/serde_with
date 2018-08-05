# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2]

### Added

* `unwrap_or_skip` allows to transparently serialize the inner part of a `Some(T)`
* Add deserialization helpser for sets and maps, inspired by [comment](https://github.com/serde-rs/serde/issues/553#issuecomment-299711855)
    * Create an error if duplicate values for a set are detected
    * Create an error if duplicate keys for a map are detected
    * Implement a first-value wins strategy for sets/maps. This is different to serde's default
        which implements a last value wins strategy.

## [0.2.1]

### Added

* Double Option pattern to differentiate between missing, unset, or existing value
* `with_prefix!` macro, which puts a prefix on every struct field

## [0.2.0]

### Added

* Add chrono support: Deserialize timestamps from int, float, and string
* Serialization of embedded JSON strings
* De/Serialization using `Display` and `FromStr` implementations
* String-based collections using `Display` and `FromStr`, allows to deserialize "#foo,#bar"

## [0.1.0]

### Added

* Reserve name on crates.io
