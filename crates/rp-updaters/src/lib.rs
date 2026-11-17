//! Updaters rewrite one file's version without touching formatting.

pub mod cargo_lock;
pub mod cargo_toml;
pub mod chart_yaml;
pub mod generic;
pub mod json_key;
pub mod package_json;
pub mod package_lock;
pub mod pyproject;
pub mod toml_key;
pub mod version_txt;
pub mod yaml_key;

mod extra_files;
mod json_scan;
mod key_path;
mod toml_util;
mod yaml_scan;

use cargo_lock::CargoLock;
use cargo_toml::CargoToml;
use chart_yaml::ChartYaml;
use package_json::PackageJson;
use package_lock::PackageLock;
use pyproject::PyProject;
use rp_core::{ExtraFile, ReleaseType};
use semver::Version;
use version_txt::VersionTxt;

/// Rewrites file content for a new version. Must preserve comments,
/// key order and indentation. Returns content unchanged if not applicable.
pub trait Updater {
    fn update(&self, content: &str, version: &Version) -> anyhow::Result<String>;
}

/// One planned file edit, relative to the repository root.
pub struct FileUpdate {
    pub path: String,
    pub updater: Box<dyn Updater>,
    /// Skip silently if the file does not exist.
    pub optional: bool,
}

/// Default files per release type for a package at `pkg_path`
/// ("." = root). Rust: Cargo.toml (+ optional Cargo.lock), Python:
/// pyproject.toml (+ optional `<pkg>/__init__.py` is NOT included; use
/// extra-files), Node: package.json (+ optional package-lock.json),
/// Helm: Chart.yaml, Simple: version.txt, Go: none.
///
/// Cargo.lock is only updated when `package_name` is given; without it the
/// updater has no names to match and is a no-op.
pub fn for_release_type(
    kind: ReleaseType,
    pkg_path: &str,
    package_name: Option<&str>,
) -> Vec<FileUpdate> {
    let at = |file: &str| join_path(pkg_path, file);
    match kind {
        ReleaseType::Rust => {
            let names: Vec<String> = package_name
                .map(|n| vec![n.to_string()])
                .unwrap_or_default();
            vec![
                required(at("Cargo.toml"), CargoToml::new()),
                optional(at("Cargo.lock"), CargoLock::new(names)),
            ]
        }
        ReleaseType::Python => vec![required(at("pyproject.toml"), PyProject)],
        ReleaseType::Node => vec![
            required(at("package.json"), PackageJson),
            optional(at("package-lock.json"), PackageLock),
        ],
        ReleaseType::Go => Vec::new(),
        ReleaseType::Helm => vec![required(at("Chart.yaml"), ChartYaml)],
        ReleaseType::Simple => vec![required(at("version.txt"), VersionTxt)],
    }
}

/// Updaters for configured extra files (paths relative to the package).
pub fn for_extra_files(files: &[ExtraFile], pkg_path: &str) -> Vec<FileUpdate> {
    extra_files::build(files, pkg_path)
}

/// Joins a package path and a file path; "." or "" means the repository root.
pub(crate) fn join_path(pkg_path: &str, file: &str) -> String {
    let base = pkg_path.trim_matches('/');
    if base.is_empty() || base == "." {
        file.to_string()
    } else {
        format!("{base}/{file}")
    }
}

fn required(path: String, updater: impl Updater + 'static) -> FileUpdate {
    FileUpdate {
        path,
        updater: Box::new(updater),
        optional: false,
    }
}

fn optional(path: String, updater: impl Updater + 'static) -> FileUpdate {
    FileUpdate {
        path,
        updater: Box::new(updater),
        optional: true,
    }
}
