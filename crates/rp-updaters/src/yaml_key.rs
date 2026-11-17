//! Extra-file YAML: the scalar at a dotted `jsonpath` (`$.` prefix optional),
//! edited line by line. Limits are documented in `yaml_scan`.

use crate::Updater;
use crate::key_path::KeyPath;
use crate::yaml_scan;
use anyhow::{Result, anyhow};
use semver::Version;

#[derive(Debug, Clone)]
pub struct YamlKey {
    key: KeyPath,
    raw: String,
}

impl YamlKey {
    pub fn new(jsonpath: &str) -> Self {
        Self {
            key: KeyPath::parse(jsonpath),
            raw: jsonpath.to_string(),
        }
    }
}

impl Updater for YamlKey {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        yaml_scan::set_scalar(content, &self.key.refs(), &version.to_string())
            .ok_or_else(|| anyhow!("no scalar value at `{}`", self.raw))
    }
}
