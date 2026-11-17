//! `.release-please-manifest.json`: package path -> current version.

use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
    pub versions: BTreeMap<String, semver::Version>,
}

impl Manifest {
    /// Load; a missing file yields an empty manifest.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let _ = path;
        todo!()
    }

    /// Serialize as pretty JSON with sorted keys and trailing newline.
    pub fn to_json(&self) -> String {
        todo!()
    }
}
