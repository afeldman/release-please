//! `.release-please-manifest.json`: package path -> current version.

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::Path;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
    pub versions: BTreeMap<String, semver::Version>,
}

impl Manifest {
    /// Load; a missing file yields an empty manifest.
    pub fn load(path: &Path) -> Result<Self> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => {
                return Err(e).with_context(|| format!("reading manifest {}", path.display()));
            }
        };
        Self::from_json(&text).with_context(|| format!("parsing manifest {}", path.display()))
    }

    fn from_json(text: &str) -> Result<Self> {
        let raw: BTreeMap<String, String> =
            serde_json::from_str(text).context("manifest must map paths to version strings")?;
        let versions = raw
            .into_iter()
            .map(|(path, version)| {
                let parsed = semver::Version::parse(&version)
                    .with_context(|| format!("invalid version {version:?} for {path:?}"))?;
                Ok((path, parsed))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        Ok(Self { versions })
    }

    /// Serialize as pretty JSON with sorted keys and trailing newline.
    pub fn to_json(&self) -> String {
        let raw: BTreeMap<&str, String> = self
            .versions
            .iter()
            .map(|(path, version)| (path.as_str(), version.to_string()))
            .collect();
        // Serializing a map of strings cannot fail.
        let json = serde_json::to_string_pretty(&raw).unwrap_or_default();
        format!("{json}\n")
    }
}
