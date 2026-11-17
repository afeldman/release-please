use super::inherit::deep_merge;
use super::*;
use serde_json::json;

fn parse(json: &str) -> Config {
    Config::from_json(json).expect("valid config")
}

fn only_package(cfg: &Config) -> &PackageConfig {
    assert_eq!(cfg.packages.len(), 1, "expected one package");
    &cfg.packages[0]
}

#[test]
fn empty_object_gives_root_package_with_defaults() {
    let cfg = parse("{}");
    let pkg = only_package(&cfg);
    assert_eq!(pkg.path, ".");
    assert_eq!(pkg.release_type, ReleaseType::Simple);
    assert_eq!(pkg.component, None);
    assert!(!pkg.bump_minor_pre_major);
    assert_eq!(pkg.changelog_sections, Some(default_sections()));
    assert_eq!(
        cfg.pull_request_title_pattern,
        "chore${scope}: release${component} ${version}"
    );
    assert_eq!(
        cfg.pull_request_header,
        ":robot: I have created a release *beep* *boop*"
    );
    assert_eq!(cfg.label, "autorelease: pending");
    assert_eq!(cfg.release_label, "autorelease: tagged");
    assert!(!cfg.separate_pull_requests);
}

#[test]
fn empty_packages_map_falls_back_to_root() {
    let cfg = parse(r#"{"release-type": "rust", "packages": {}}"#);
    let pkg = only_package(&cfg);
    assert_eq!(pkg.path, ".");
    assert_eq!(pkg.release_type, ReleaseType::Rust);
}

#[test]
fn top_level_pr_options_are_read() {
    let cfg = parse(
        r#"{
            "pull-request-title-pattern": "release: ${version}",
            "pull-request-header": "hdr",
            "label": "pending",
            "release-label": "tagged",
            "separate-pull-requests": true
        }"#,
    );
    assert_eq!(cfg.pull_request_title_pattern, "release: ${version}");
    assert_eq!(cfg.pull_request_header, "hdr");
    assert_eq!(cfg.label, "pending");
    assert_eq!(cfg.release_label, "tagged");
    assert!(cfg.separate_pull_requests);
}

#[test]
fn non_string_pr_option_uses_default() {
    let cfg = parse(r#"{"label": 5}"#);
    assert_eq!(cfg.label, "autorelease: pending");
}

#[test]
fn package_inherits_top_level_options() {
    let cfg = parse(
        r#"{
            "release-type": "node",
            "bump-minor-pre-major": true,
            "packages": {"web": {"component": "frontend"}}
        }"#,
    );
    let pkg = only_package(&cfg);
    assert_eq!(pkg.path, "web");
    assert_eq!(pkg.release_type, ReleaseType::Node);
    assert!(pkg.bump_minor_pre_major);
    assert_eq!(pkg.component.as_deref(), Some("frontend"));
}

#[test]
fn package_overrides_top_level_scalar() {
    let cfg = parse(
        r#"{
            "bump-minor-pre-major": true,
            "packages": {"a": {"bump-minor-pre-major": false}}
        }"#,
    );
    assert!(!only_package(&cfg).bump_minor_pre_major);
}

#[test]
fn package_array_replaces_top_level_array() {
    let cfg = parse(
        r#"{
            "extra-files": ["top.txt"],
            "packages": {"a": {"extra-files": ["own.txt"]}}
        }"#,
    );
    assert_eq!(
        only_package(&cfg).extra_files,
        vec![ExtraFile::Path("own.txt".into())]
    );
}

#[test]
fn packages_are_sorted_by_path_and_keep_root_dot() {
    let cfg =
        parse(r#"{"packages": {"crates/b": {}, ".": {"release-type": "rust"}, "crates/a": {}}}"#);
    let paths: Vec<&str> = cfg.packages.iter().map(|p| p.path.as_str()).collect();
    assert_eq!(paths, [".", "crates/a", "crates/b"]);
    assert_eq!(cfg.packages[0].release_type, ReleaseType::Rust);
    assert_eq!(cfg.packages[1].release_type, ReleaseType::Simple);
}

#[test]
fn top_level_tag_options_are_inherited() {
    let cfg = parse(
        r#"{
            "include-v-in-tag": false,
            "tag-separator": "_",
            "packages": {"crates/t": {}}
        }"#,
    );
    let pkg = only_package(&cfg);
    assert_eq!(pkg.include_v_in_tag, Some(false));
    assert_eq!(pkg.tag_separator.as_deref(), Some("_"));
}

#[test]
fn prerelease_options_are_inherited() {
    let cfg = parse(r#"{"prerelease-type": "rc", "packages": {"p": {}}}"#);
    assert_eq!(only_package(&cfg).prerelease_type.as_deref(), Some("rc"));
}

#[test]
fn unknown_keys_are_ignored() {
    let cfg = parse(r#"{"no-such-key": [1, 2], "packages": {"p": {"also-unknown": true}}}"#);
    assert_eq!(only_package(&cfg).path, "p");
}

#[test]
fn custom_changelog_sections_are_kept() {
    let cfg =
        parse(r#"{"changelog-sections": [{"type": "feat", "section": "New", "hidden": false}]}"#);
    assert_eq!(
        only_package(&cfg).changelog_sections,
        Some(vec![ChangelogSection {
            kind: "feat".into(),
            section: "New".into(),
            hidden: false,
        }])
    );
}

#[test]
fn default_sections_match_release_please_order_and_visibility() {
    let sections = default_sections();
    let got: Vec<(&str, &str, bool)> = sections
        .iter()
        .map(|s| (s.kind.as_str(), s.section.as_str(), s.hidden))
        .collect();
    assert_eq!(
        got,
        vec![
            ("feat", "Features", false),
            ("fix", "Bug Fixes", false),
            ("perf", "Performance Improvements", false),
            ("revert", "Reverts", false),
            ("docs", "Documentation", true),
            ("style", "Styles", true),
            ("chore", "Miscellaneous Chores", true),
            ("refactor", "Code Refactoring", true),
            ("test", "Tests", true),
            ("build", "Build System", true),
            ("ci", "Continuous Integration", true),
        ]
    );
}

#[test]
fn invalid_release_type_is_an_error() {
    assert!(Config::from_json(r#"{"release-type": "cobol"}"#).is_err());
}

#[test]
fn invalid_json_is_an_error() {
    assert!(Config::from_json("{").is_err());
}

#[test]
fn non_object_root_is_an_error() {
    assert!(Config::from_json("[]").is_err());
}

#[test]
fn packages_must_be_an_object() {
    assert!(Config::from_json(r#"{"packages": ["a"]}"#).is_err());
}

#[test]
fn load_missing_file_is_an_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    assert!(Config::load(&dir.path().join("release-please-config.json")).is_err());
}

#[test]
fn load_reads_file_from_disk() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("release-please-config.json");
    std::fs::write(&path, r#"{"release-type": "python"}"#).expect("write");
    let cfg = Config::load(&path).expect("load");
    assert_eq!(only_package(&cfg).release_type, ReleaseType::Python);
}

#[test]
fn deep_merge_merges_objects_recursively() {
    let mut base = json!({"a": {"x": 1, "y": 2}, "b": 1});
    deep_merge(&mut base, json!({"a": {"y": 9, "z": 3}}));
    assert_eq!(base, json!({"a": {"x": 1, "y": 9, "z": 3}, "b": 1}));
}

#[test]
fn deep_merge_replaces_non_object_values() {
    let mut base = json!({"a": [1, 2], "b": "old"});
    deep_merge(&mut base, json!({"a": [3], "b": {"new": true}}));
    assert_eq!(base, json!({"a": [3], "b": {"new": true}}));
}
