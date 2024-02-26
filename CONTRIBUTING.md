# Contributing to `serde_with`

1. [I Have a Question](#i-have-a-question)
2. [Reporting Bugs](#reporting-bugs)
3. [Submitting a PR](#submitting-a-pr)
4. [Adding support for new crates](#adding-support-for-new-crates)

## I Have a Question

Check out the [user guide][user guide] to find out more tips and tricks about this crate.

For further help using this crate, you can [open a new discussion](https://github.com/jonasbb/serde_with/discussions/new) or ask on [users.rust-lang.org](https://users.rust-lang.org/).
For bugs, please open a [new issue](https://github.com/jonasbb/serde_with/issues/new) on GitHub.

## Reporting Bugs

A good bug report shouldn't leave others needing to chase you up for more information.
Make sure to include the three major parts of information:

1. Show your code and serialized data.
    Without them, it is almost impossible to understand where the problem arises.
2. Explain what the expected result and/or expected serialized data is.
3. If possible, prepare a minimal running example.

Security vulnerabilities should be reported privately as [security advisory](https://github.com/jonasbb/serde_with/security).
Check [SECURITY.md](./SECURITY.md) for details.

## Submitting a PR

This repository provides a dev container setup, which can be used with VS Code or GitHub Codespaces to simplify contributing.

1. Start by opening a PR.
2. New features should always be accompanied by tests and documentation.
    * New transformations should have documentation using rustdoc.
      A short description should be added to `serde_as_transformations.md` too.
    * Integration tests belong in the `tests` folder.
    * A changelog entry can also be added.
3. Contributions must pass `cargo clippy` and `cargo fmt`.
    This is also checked by CI and needs to pass before the PR can be merged.
    The CI does not allow any warnings in the compiled code.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.

[user guide]: https://docs.rs/serde_with/latest/serde_with/guide/index.html

## Adding support for new crates

`serde_with` takes a liberal approach to crate compatibility and provides additional implementations for well-known crates, such as `chrono` or `indexmap`.
New crates should always be optional and only be enabled with an explicit feature.
The feature name should include the major version of the crate.
This allows adding support for new breaking releases of these crates.
For example, `chrono` is currently at v0.4, so the feature name should be `chrono_0_4`.
A new chrono v0.5 would get the feature name `chrono_0_5` and the v1 release `chrono_1`.

`serde_with` depends also on further crates to implement the converters, such as `base64` or `hex`.
Here the feature name should not depend on the crate, but rather describe the added functionality enabled by it.
For example, additional JSON functionality is behind the `json` feature, and it depends on `serde_json` for the implementation.
