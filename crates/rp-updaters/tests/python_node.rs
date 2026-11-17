//! pyproject.toml, package.json and package-lock.json updaters.

use rp_updaters::Updater;
use rp_updaters::package_json::PackageJson;
use rp_updaters::package_lock::PackageLock;
use rp_updaters::pyproject::PyProject;
use semver::Version;

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid test version")
}

#[test]
fn pyproject_project_version_keeps_comments() {
    let input = "\
# build config
[build-system]
requires = [\"setuptools>=68\"]

[project]
name = \"demo\"   # package
version  =  \"0.3.1\"  # bumped by release
description = \"x\"
";
    let expected = "\
# build config
[build-system]
requires = [\"setuptools>=68\"]

[project]
name = \"demo\"   # package
version  =  \"0.4.0\"  # bumped by release
description = \"x\"
";
    assert_eq!(
        PyProject.update(input, &v("0.4.0")).expect("update"),
        expected
    );
}

#[test]
fn pyproject_poetry_version_is_used_as_fallback() {
    let input = "[tool.poetry]\nname = \"demo\"\nversion = '0.3.1'\n";
    let expected = "[tool.poetry]\nname = \"demo\"\nversion = '0.4.0'\n";
    assert_eq!(
        PyProject.update(input, &v("0.4.0")).expect("update"),
        expected
    );
}

#[test]
fn pyproject_prefers_project_over_poetry() {
    let input = "[project]\nversion = \"1.0.0\"\n\n[tool.poetry]\nversion = \"1.0.0\"\n";
    let expected = "[project]\nversion = \"1.1.0\"\n\n[tool.poetry]\nversion = \"1.0.0\"\n";
    assert_eq!(
        PyProject.update(input, &v("1.1.0")).expect("update"),
        expected
    );
}

#[test]
fn pyproject_dynamic_version_is_skipped() {
    let input = "[project]\nname = \"demo\"\ndynamic = [\"version\", \"readme\"]\n\n[tool.setuptools.dynamic]\nversion = { attr = \"demo.__version__\" }\n";
    assert_eq!(PyProject.update(input, &v("2.0.0")).expect("update"), input);
}

#[test]
fn pyproject_without_version_is_an_error() {
    let input = "[project]\nname = \"demo\"\n";
    assert!(PyProject.update(input, &v("1.0.0")).is_err());
}

#[test]
fn package_json_keeps_tab_indent_and_key_order() {
    let input = "{\n\t\"name\": \"demo\",\n\t\"version\": \"1.2.3\",\n\t\"private\": true\n}\n";
    let expected = "{\n\t\"name\": \"demo\",\n\t\"version\": \"1.3.0\",\n\t\"private\": true\n}\n";
    assert_eq!(
        PackageJson.update(input, &v("1.3.0")).expect("update"),
        expected
    );
}

#[test]
fn package_json_four_space_indent_is_kept() {
    let input = "{\n    \"name\": \"demo\",\n    \"version\": \"0.0.1\"\n}";
    let expected = "{\n    \"name\": \"demo\",\n    \"version\": \"0.1.0\"\n}";
    assert_eq!(
        PackageJson.update(input, &v("0.1.0")).expect("update"),
        expected
    );
}

#[test]
fn package_json_nested_version_before_top_level_is_untouched() {
    let input = "{\n  \"dependencies\": {\n    \"version\": \"9.9.9\",\n    \"inner\": { \"version\": \"8.8.8\" }\n  },\n  \"version\": \"1.2.3\"\n}\n";
    let expected = "{\n  \"dependencies\": {\n    \"version\": \"9.9.9\",\n    \"inner\": { \"version\": \"8.8.8\" }\n  },\n  \"version\": \"2.0.0\"\n}\n";
    assert_eq!(
        PackageJson.update(input, &v("2.0.0")).expect("update"),
        expected
    );
}

#[test]
fn package_json_string_with_braces_does_not_confuse_scanner() {
    let input = "{\n  \"description\": \"a } weird \\\" { value\",\n  \"version\": \"1.0.0\"\n}\n";
    let expected =
        "{\n  \"description\": \"a } weird \\\" { value\",\n  \"version\": \"1.0.1\"\n}\n";
    assert_eq!(
        PackageJson.update(input, &v("1.0.1")).expect("update"),
        expected
    );
}

#[test]
fn package_json_without_version_is_an_error() {
    assert!(
        PackageJson
            .update("{\n  \"name\": \"demo\"\n}\n", &v("1.0.0"))
            .is_err()
    );
}

const LOCK: &str = "\
{
  \"name\": \"demo\",
  \"version\": \"1.2.3\",
  \"lockfileVersion\": 3,
  \"packages\": {
    \"\": {
      \"name\": \"demo\",
      \"version\": \"1.2.3\",
      \"license\": \"MIT\"
    },
    \"node_modules/x\": {
      \"version\": \"1.2.3\"
    }
  }
}
";

#[test]
fn package_lock_updates_root_and_empty_package_entry_only() {
    let expected = "\
{
  \"name\": \"demo\",
  \"version\": \"1.3.0\",
  \"lockfileVersion\": 3,
  \"packages\": {
    \"\": {
      \"name\": \"demo\",
      \"version\": \"1.3.0\",
      \"license\": \"MIT\"
    },
    \"node_modules/x\": {
      \"version\": \"1.2.3\"
    }
  }
}
";
    assert_eq!(
        PackageLock.update(LOCK, &v("1.3.0")).expect("update"),
        expected
    );
}

#[test]
fn package_lock_v1_without_root_entry_updates_top_level() {
    let input =
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.2.3\",\n  \"lockfileVersion\": 1\n}\n";
    let expected =
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.2.4\",\n  \"lockfileVersion\": 1\n}\n";
    assert_eq!(
        PackageLock.update(input, &v("1.2.4")).expect("update"),
        expected
    );
}

#[test]
fn package_lock_without_any_version_is_an_error() {
    assert!(
        PackageLock
            .update("{\n  \"packages\": {}\n}\n", &v("1.0.0"))
            .is_err()
    );
}
