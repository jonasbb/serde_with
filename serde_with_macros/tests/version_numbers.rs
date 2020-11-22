#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn test_changelog() {
    version_sync::assert_contains_regex!("CHANGELOG.md", r#"## \[{version}\]"#);
}
