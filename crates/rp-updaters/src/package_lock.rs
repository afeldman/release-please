//! `package-lock.json`: top-level `"version"` and `packages[""].version`
//! (the root package entry). Each one is updated if present.

use crate::Updater;
use crate::json_scan;
use anyhow::{Result, bail};
use semver::Version;

#[derive(Debug, Clone, Copy, Default)]
pub struct PackageLock;

impl Updater for PackageLock {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let v = version.to_string();
        let mut out = content.to_string();
        let mut found = false;
        for path in [&["version"][..], &["packages", "", "version"][..]] {
            if let Some(next) = json_scan::set_string(&out, path, &v) {
                out = next;
                found = true;
            }
        }
        if !found {
            bail!("package-lock.json has no string \"version\" to update");
        }
        Ok(out)
    }
}
