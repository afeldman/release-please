//! Cargo.toml and Cargo.lock updaters: exact output on realistic fixtures.

use rp_updaters::Updater;
use rp_updaters::cargo_lock::CargoLock;
use rp_updaters::cargo_toml::CargoToml;
use semver::Version;

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid test version")
}

fn toml(content: &str, to: &str) -> String {
    CargoToml::new()
        .update(content, &v(to))
        .expect("update succeeds")
}

#[test]
fn package_version_keeps_comments_and_odd_spacing() {
    let input = "\
# top comment
[package]
name = \"demo\"            # crate name
version   =  \"0.1.0\"  # current
edition = \"2024\"

[dependencies]
serde = \"1\"
";
    let expected = "\
# top comment
[package]
name = \"demo\"            # crate name
version   =  \"0.2.0\"  # current
edition = \"2024\"

[dependencies]
serde = \"1\"
";
    assert_eq!(toml(input, "0.2.0"), expected);
}

#[test]
fn workspace_inherited_package_version_is_skipped() {
    let input = "[package]\nname = \"x\"\nversion.workspace = true\n";
    assert_eq!(toml(input, "9.0.0"), input);
    let inline = "[package]\nname = \"x\"\nversion = { workspace = true }\n";
    assert_eq!(toml(inline, "9.0.0"), inline);
}

#[test]
fn workspace_root_package_version_is_bumped() {
    let input = "\
[workspace]
members = [\"crates/*\"]

[workspace.package]
version = \"1.0.0\"
edition = \"2024\"
";
    let expected = "\
[workspace]
members = [\"crates/*\"]

[workspace.package]
version = \"1.1.0\"
edition = \"2024\"
";
    assert_eq!(toml(input, "1.1.0"), expected);
}

#[test]
fn path_dependencies_with_version_are_bumped() {
    let input = "\
[package]
name = \"demo\"
version = \"0.1.0\"

[dependencies]
core = { path = \"../core\", version = \"0.1.0\" }  # local
serde = \"1\"
other = { path = \"../other\" }
registry = { version = \"2.0\" }
";
    let expected = "\
[package]
name = \"demo\"
version = \"0.2.0\"

[dependencies]
core = { path = \"../core\", version = \"0.2.0\" }  # local
serde = \"1\"
other = { path = \"../other\" }
registry = { version = \"2.0\" }
";
    assert_eq!(toml(input, "0.2.0"), expected);
}

#[test]
fn dev_dependency_table_form_is_bumped() {
    let input = "\
[package]
name = \"demo\"
version = \"0.1.0\"

[dev-dependencies.helper]
path = \"../helper\"
version = \"0.1.0\"
";
    let expected = "\
[package]
name = \"demo\"
version = \"0.2.0\"

[dev-dependencies.helper]
path = \"../helper\"
version = \"0.2.0\"
";
    assert_eq!(toml(input, "0.2.0"), expected);
}

#[test]
fn workspace_dependencies_are_bumped() {
    let input = "\
[workspace]
members = [\"crates/*\"]

[workspace.package]
version = \"1.0.0\"

[workspace.dependencies]
rp-core = { path = \"crates/rp-core\", version = \"1.0.0\" }
anyhow = \"1\"
";
    let expected = "\
[workspace]
members = [\"crates/*\"]

[workspace.package]
version = \"1.1.0\"

[workspace.dependencies]
rp-core = { path = \"crates/rp-core\", version = \"1.1.0\" }
anyhow = \"1\"
";
    assert_eq!(toml(input, "1.1.0"), expected);
}

#[test]
fn build_and_target_dependencies_are_bumped() {
    let input = "\
[package]
name = \"demo\"
version = \"0.1.0\"

[build-dependencies]
gen = { path = \"../gen\", version = \"0.1.0\" }

[target.'cfg(unix)'.dependencies]
helper = { path = \"../helper\", version = \"0.1.0\" }
";
    let out = toml(input, "0.3.0");
    assert!(
        out.contains("gen = { path = \"../gen\", version = \"0.3.0\" }"),
        "{out}"
    );
    assert!(
        out.contains("helper = { path = \"../helper\", version = \"0.3.0\" }"),
        "{out}"
    );
    assert!(
        out.contains("version = \"0.3.0\"\n\n[build-dependencies]"),
        "{out}"
    );
}

#[test]
fn prerelease_version_is_written_in_full() {
    let input = "[package]\nname = \"demo\"\nversion = \"1.0.0\"\n";
    assert_eq!(
        toml(input, "2.0.0-rc.1"),
        "[package]\nname = \"demo\"\nversion = \"2.0.0-rc.1\"\n"
    );
}

#[test]
fn literal_quote_style_is_kept() {
    let input = "[package]\nname = 'demo'\nversion = '1.0.0'\n";
    assert_eq!(
        toml(input, "1.0.1"),
        "[package]\nname = 'demo'\nversion = '1.0.1'\n"
    );
}

#[test]
fn with_deps_limits_dependency_bumps() {
    let input = "\
[package]
version = \"0.1.0\"

[dependencies]
core = { path = \"../core\", version = \"0.1.0\" }
other = { path = \"../other\", version = \"0.1.0\" }
";
    let out = CargoToml::with_deps(["core"])
        .update(input, &v("0.2.0"))
        .expect("update succeeds");
    let expected = "\
[package]
version = \"0.2.0\"

[dependencies]
core = { path = \"../core\", version = \"0.2.0\" }
other = { path = \"../other\", version = \"0.1.0\" }
";
    assert_eq!(out, expected);
}

#[test]
fn virtual_manifest_without_versions_is_unchanged() {
    let input = "[workspace]\nmembers = [\"a\"]\n\n[workspace.dependencies]\nserde = \"1\"\n";
    assert_eq!(toml(input, "3.0.0"), input);
}

#[test]
fn invalid_toml_is_an_error() {
    assert!(
        CargoToml::new()
            .update("[package\nversion = ", &v("1.0.0"))
            .is_err()
    );
}

const LOCK: &str = "\
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = \"anyhow\"
version = \"1.0.80\"
source = \"registry+https://github.com/rust-lang/crates.io-index\"
checksum = \"abc\"

[[package]]
name = \"demo\"
version = \"0.1.0\"
dependencies = [
 \"anyhow\",
 \"demo-core\",
]

[[package]]
name = \"demo-core\"
version = \"0.1.0\"

[metadata]
";

#[test]
fn cargo_lock_updates_only_named_packages() {
    let out = CargoLock::new(["demo"])
        .update(LOCK, &v("0.2.0"))
        .expect("update succeeds");
    let expected = LOCK.replacen(
        "name = \"demo\"\nversion = \"0.1.0\"",
        "name = \"demo\"\nversion = \"0.2.0\"",
        1,
    );
    assert_eq!(out, expected);
    assert!(out.contains("name = \"demo-core\"\nversion = \"0.1.0\""));
}

#[test]
fn cargo_lock_updates_several_names() {
    let out = CargoLock::new(["demo", "demo-core"])
        .update(LOCK, &v("0.2.0"))
        .expect("update succeeds");
    assert_eq!(out.matches("version = \"0.2.0\"").count(), 2);
    assert!(out.contains("version = \"1.0.80\""));
}

#[test]
fn cargo_lock_empty_name_list_is_noop() {
    let out = CargoLock::new(Vec::<String>::new())
        .update(LOCK, &v("0.2.0"))
        .expect("update succeeds");
    assert_eq!(out, LOCK);
}

#[test]
fn cargo_lock_keeps_crlf_line_endings() {
    let input = "[[package]]\r\nname = \"demo\"\r\nversion = \"0.1.0\"\r\n\r\n[[package]]\r\nname = \"x\"\r\nversion = \"0.1.0\"\r\n";
    let expected = "[[package]]\r\nname = \"demo\"\r\nversion = \"0.2.0\"\r\n\r\n[[package]]\r\nname = \"x\"\r\nversion = \"0.1.0\"\r\n";
    let out = CargoLock::new(["demo"])
        .update(input, &v("0.2.0"))
        .expect("update succeeds");
    assert_eq!(out, expected);
}

#[test]
fn cargo_lock_dependency_lines_are_not_touched() {
    let input =
        "[[package]]\nname = \"app\"\nversion = \"0.1.0\"\ndependencies = [\n \"demo 0.1.0\",\n]\n";
    let out = CargoLock::new(["demo"])
        .update(input, &v("0.2.0"))
        .expect("update succeeds");
    assert_eq!(out, input);
}
