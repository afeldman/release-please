//! Updaters rewrite one file's version without touching formatting.

use rp_core::{ExtraFile, ReleaseType};
use semver::Version;

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
pub fn for_release_type(kind: ReleaseType, pkg_path: &str, package_name: Option<&str>) -> Vec<FileUpdate> {
    let _ = (kind, pkg_path, package_name);
    todo!()
}

/// Updaters for configured extra files (paths relative to the package).
pub fn for_extra_files(files: &[ExtraFile], pkg_path: &str) -> Vec<FileUpdate> {
    let _ = (files, pkg_path);
    todo!()
}
