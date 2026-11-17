//! `package.json`: top-level `"version"` only. The value is rewritten in
//! place, so key order, indentation and the rest of the file are unchanged.

use crate::Updater;
use crate::json_scan;
use anyhow::{Result, anyhow};
use semver::Version;

#[derive(Debug, Clone, Copy, Default)]
pub struct PackageJson;

impl Updater for PackageJson {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        json_scan::set_string(content, &["version"], &version.to_string())
            .ok_or_else(|| anyhow!("package.json has no top-level string \"version\""))
    }
}
