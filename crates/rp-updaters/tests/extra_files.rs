//! Typed extra files (json/toml/yaml keys) and the lib.rs mappings.

use rp_core::{ExtraFile, ReleaseType};
use rp_updaters::json_key::JsonKey;
use rp_updaters::toml_key::TomlKey;
use rp_updaters::yaml_key::YamlKey;
use rp_updaters::{Updater, for_extra_files, for_release_type};
use semver::Version;

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid test version")
}

fn typed(kind: &str, path: &str, jsonpath: Option<&str>) -> ExtraFile {
    ExtraFile::Typed {
        kind: kind.to_string(),
        path: path.to_string(),
        jsonpath: jsonpath.map(str::to_string),
    }
}

#[test]
fn json_key_updates_nested_value_only() {
    let input = "{\n  \"package\": { \"version\": \"1.0.0\" },\n  \"version\": \"9.9.9\"\n}\n";
    let expected = "{\n  \"package\": { \"version\": \"2.0.0\" },\n  \"version\": \"9.9.9\"\n}\n";
    let out = JsonKey::new("$.package.version")
        .update(input, &v("2.0.0"))
        .expect("update");
    assert_eq!(out, expected);
}

#[test]
fn json_key_without_root_prefix_works() {
    let input = "{\"a\":{\"b\":\"1.0.0\"}}";
    let out = JsonKey::new("a.b")
        .update(input, &v("1.0.1"))
        .expect("update");
    assert_eq!(out, "{\"a\":{\"b\":\"1.0.1\"}}");
}

#[test]
fn json_key_missing_path_is_an_error() {
    assert!(
        JsonKey::new("$.nope.version")
            .update("{\"a\": 1}", &v("1.0.0"))
            .is_err()
    );
}

#[test]
fn json_key_non_string_value_is_an_error() {
    assert!(
        JsonKey::new("$.version")
            .update("{\"version\": 1}", &v("1.0.0"))
            .is_err()
    );
}

#[test]
fn toml_key_updates_nested_table_keeping_comment() {
    let input = "[tool.demo]\nversion = \"1.0.0\"   # keep me\nname = \"demo\"\n";
    let expected = "[tool.demo]\nversion = \"2.0.0\"   # keep me\nname = \"demo\"\n";
    let out = TomlKey::new("$.tool.demo.version")
        .update(input, &v("2.0.0"))
        .expect("update");
    assert_eq!(out, expected);
}

#[test]
fn toml_key_missing_path_is_an_error() {
    assert!(
        TomlKey::new("$.tool.demo.version")
            .update("[tool]\n", &v("1.0.0"))
            .is_err()
    );
}

#[test]
fn yaml_key_updates_nested_quoted_scalar_keeping_comment() {
    let input = "image:\n  repository: x\n  tag: \"1.0.0\"  # pinned\nother: 2\n";
    let expected = "image:\n  repository: x\n  tag: \"2.0.0\"  # pinned\nother: 2\n";
    let out = YamlKey::new("$.image.tag")
        .update(input, &v("2.0.0"))
        .expect("update");
    assert_eq!(out, expected);
}

#[test]
fn yaml_key_plain_scalar_at_root() {
    let input = "name: demo\nversion: 0.1.0 # chart\n";
    let expected = "name: demo\nversion: 0.2.0 # chart\n";
    assert_eq!(
        YamlKey::new("version")
            .update(input, &v("0.2.0"))
            .expect("update"),
        expected
    );
}

#[test]
fn yaml_key_does_not_leave_its_block() {
    let input = "image:\n  tag: 1.0.0\nversion: 1.0.0\n";
    let out = YamlKey::new("$.image.version").update(input, &v("2.0.0"));
    assert!(out.is_err());
}

#[test]
fn yaml_key_missing_is_an_error() {
    assert!(
        YamlKey::new("$.nope")
            .update("a: 1\n", &v("1.0.0"))
            .is_err()
    );
}

#[test]
fn extra_file_plain_path_uses_generic_markers_and_joins_package_path() {
    let files = vec![ExtraFile::Path("src/version.rs".to_string())];
    let updates = for_extra_files(&files, "crates/a/");
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].path, "crates/a/src/version.rs");
    assert!(!updates[0].optional);
    let out = updates[0]
        .updater
        .update(
            "const V: &str = \"1.0.0\"; // x-release-please-version\n",
            &v("1.1.0"),
        )
        .expect("update");
    assert_eq!(
        out,
        "const V: &str = \"1.1.0\"; // x-release-please-version\n"
    );
}

#[test]
fn extra_file_typed_routes_by_kind() {
    let files = vec![
        typed("json", "a.json", Some("$.version")),
        typed("toml", "b.toml", Some("$.version")),
        typed("yaml", "c.yaml", Some("$.version")),
        typed("generic", "d.txt", None),
    ];
    let updates = for_extra_files(&files, ".");
    let paths: Vec<&str> = updates.iter().map(|u| u.path.as_str()).collect();
    assert_eq!(paths, vec!["a.json", "b.toml", "c.yaml", "d.txt"]);
    assert_eq!(
        updates[0]
            .updater
            .update("{\"version\": \"1.0.0\"}", &v("1.0.1"))
            .expect("json"),
        "{\"version\": \"1.0.1\"}"
    );
    assert_eq!(
        updates[1]
            .updater
            .update("version = \"1.0.0\"\n", &v("1.0.1"))
            .expect("toml"),
        "version = \"1.0.1\"\n"
    );
    assert_eq!(
        updates[2]
            .updater
            .update("version: 1.0.0\n", &v("1.0.1"))
            .expect("yaml"),
        "version: 1.0.1\n"
    );
}

#[test]
fn extra_file_typed_without_jsonpath_fails_on_update() {
    let files = vec![typed("json", "a.json", None)];
    let updates = for_extra_files(&files, ".");
    let err = updates[0]
        .updater
        .update("{}", &v("1.0.0"))
        .expect_err("config error");
    assert!(err.to_string().contains("jsonpath"), "{err}");
}

#[test]
fn extra_file_unknown_kind_fails_on_update() {
    let files = vec![typed("xml", "a.xml", Some("x"))];
    let updates = for_extra_files(&files, ".");
    let err = updates[0]
        .updater
        .update("<x/>", &v("1.0.0"))
        .expect_err("config error");
    assert!(err.to_string().contains("unknown extra file type"), "{err}");
}

#[test]
fn release_type_rust_maps_cargo_files_with_package_name() {
    let updates = for_release_type(ReleaseType::Rust, "crates/a", Some("a"));
    let summary: Vec<(&str, bool)> = updates
        .iter()
        .map(|u| (u.path.as_str(), u.optional))
        .collect();
    assert_eq!(
        summary,
        vec![
            ("crates/a/Cargo.toml", false),
            ("crates/a/Cargo.lock", true)
        ]
    );

    let lock = &updates[1].updater;
    let content = "[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n";
    assert_eq!(
        lock.update(content, &v("0.2.0")).expect("lock"),
        content.replace("0.1.0", "0.2.0")
    );
}

#[test]
fn release_type_rust_without_package_name_makes_lock_a_noop() {
    let updates = for_release_type(ReleaseType::Rust, ".", None);
    let content = "[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n";
    assert_eq!(
        updates[1]
            .updater
            .update(content, &v("0.2.0"))
            .expect("lock"),
        content
    );
    assert_eq!(updates[1].path, "Cargo.lock");
}

#[test]
fn release_type_node_has_optional_lock() {
    let updates = for_release_type(ReleaseType::Node, "web", None);
    let summary: Vec<(&str, bool)> = updates
        .iter()
        .map(|u| (u.path.as_str(), u.optional))
        .collect();
    assert_eq!(
        summary,
        vec![("web/package.json", false), ("web/package-lock.json", true)]
    );
}

#[test]
fn release_type_single_file_kinds() {
    let cases = [
        (ReleaseType::Python, ".", "pyproject.toml"),
        (ReleaseType::Helm, "chart", "chart/Chart.yaml"),
        (ReleaseType::Simple, "pkg", "pkg/version.txt"),
    ];
    for (kind, pkg, expected) in cases {
        let updates = for_release_type(kind, pkg, None);
        assert_eq!(updates.len(), 1, "{kind:?}");
        assert_eq!(updates[0].path, expected);
        assert!(!updates[0].optional);
    }
}

#[test]
fn release_type_go_has_no_files() {
    assert!(for_release_type(ReleaseType::Go, "svc", None).is_empty());
}
