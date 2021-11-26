// Needed to supress a 2021 incompatability warning in the macro generated code
// The non_fmt_panic lint is not yet available on most Rust versions
#![allow(unknown_lints, non_fmt_panics)]

use version_sync::{
    assert_contains_regex, assert_html_root_url_updated, assert_markdown_deps_updated,
};

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_changelog() {
    assert_contains_regex!("CHANGELOG.md", r#"## \[{version}\]"#);
}

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}
