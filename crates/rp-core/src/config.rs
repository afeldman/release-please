//! `release-please-config.json` (compatible subset).
//!
//! Top-level package options are defaults; each entry in `packages`
//! (keyed by path, "." for the root) overrides them.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseType {
    /// Cargo.toml (+ workspace members, Cargo.lock).
    Rust,
    /// pyproject.toml (`project.version` or `tool.poetry.version`).
    Python,
    /// package.json (+ package-lock.json root entries).
    Node,
    /// Go: version lives only in the tag.
    Go,
    /// Helm Chart.yaml `version` (and `appVersion` if present).
    Helm,
    /// version.txt only.
    #[default]
    Simple,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ChangelogSection {
    #[serde(rename = "type")]
    pub kind: String,
    pub section: String,
    #[serde(default)]
    pub hidden: bool,
}

/// Extra file to bump. A plain string means `generic` (marker comments).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum ExtraFile {
    Path(String),
    Typed {
        #[serde(rename = "type")]
        kind: String, // "generic" | "json" | "toml" | "yaml"
        path: String,
        /// Dotted key path for json/toml/yaml, e.g. "package.version".
        #[serde(default)]
        jsonpath: Option<String>,
    },
}

/// Fully resolved options for one package.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct PackageConfig {
    #[serde(skip)]
    pub path: String,
    pub release_type: ReleaseType,
    pub component: Option<String>,
    pub package_name: Option<String>,
    pub changelog_path: Option<String>,
    pub changelog_sections: Option<Vec<ChangelogSection>>,
    pub extra_files: Vec<ExtraFile>,
    pub bump_minor_pre_major: bool,
    pub bump_patch_for_minor_pre_major: bool,
    pub include_component_in_tag: Option<bool>,
    pub include_v_in_tag: Option<bool>,
    pub tag_separator: Option<String>,
    pub release_as: Option<String>,
    pub initial_version: Option<String>,
    pub prerelease_type: Option<String>,
    pub prerelease: bool,
    pub draft: bool,
    pub skip_changelog: bool,
    pub exclude_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub packages: Vec<PackageConfig>,
    pub pull_request_title_pattern: String,
    pub pull_request_header: String,
    pub label: String,
    pub release_label: String,
    pub separate_pull_requests: bool,
}

impl Config {
    /// Load and resolve inheritance. Defaults:
    /// title "chore${scope}: release${component} ${version}",
    /// label "autorelease: pending", release label "autorelease: tagged",
    /// default changelog sections = feat/fix/perf/revert visible,
    /// others hidden (see [`default_sections`]).
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let _ = path;
        todo!()
    }

    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        let _ = json;
        todo!()
    }
}

/// Default sections in release-please order.
pub fn default_sections() -> Vec<ChangelogSection> {
    todo!()
}

#[allow(dead_code)]
type RawPackages = BTreeMap<String, serde_json::Value>;
