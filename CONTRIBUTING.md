# Contributing to `serde_with`

1. [I Have a Question](#i-have-a-question)
2. [Reporting Bugs](#reporting-bugs)
3. [Submitting a PR](#submitting-a-pr)

## I Have a Question

Check out the [user guide][user guide] to find out more tips and tricks about this crate.

For further help using this crate you can [open a new discussion](https://github.com/jonasbb/serde_with/discussions/new) or ask on [users.rust-lang.org](https://users.rust-lang.org/).
For bugs please open a [new issue](https://github.com/jonasbb/serde_with/issues/new) on Github.

## Reporting Bugs

A good bug report shouldn't leave others needing to chase you up for more information.
Make sure to include the three major parts of information:

1. Show your code and serialized data.
    Without them it is almost impossible to understand where the problem arises.
2. Explain what the expected result and/or expected serialized data is.
3. If possible prepare a minimal running example.

Security vulnerabilities should be reported like normal bugs in the issue tracker.

## Submitting a PR

This repository provides a devcontainer setup, which can be used with VS Code or GitHub codespaces to simplify contributions.

1. Simply start by opening a PR.
2. New features should always be accompanied by tests and documentation.
    * New transformations should have a documentation using rustdoc. A short descriptions should be added to `serde_as_transformations.md` too.
    * Integration tests belong in the `tests` folder.
    * A changelog entry can also be added.
3. Contributions must pass `cargo clippy` and `cargo fmt`.
    This is also checked by CI and needs to pass before the PR can be merged.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[user guide]: https://docs.rs/serde_with/latest/serde_with/guide/index.html
