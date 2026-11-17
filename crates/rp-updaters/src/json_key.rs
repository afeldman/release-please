//! Extra-file JSON: the string at a dotted `jsonpath` (`$.` prefix optional).
//! Only that value is rewritten; see `json_scan` for the matching rules.

use crate::Updater;
use crate::json_scan;
use crate::key_path::KeyPath;
use anyhow::{Result, anyhow};
use semver::Version;

#[derive(Debug, Clone)]
pub struct JsonKey {
    key: KeyPath,
    raw: String,
}

impl JsonKey {
    pub fn new(jsonpath: &str) -> Self {
        Self {
            key: KeyPath::parse(jsonpath),
            raw: jsonpath.to_string(),
        }
    }
}

impl Updater for JsonKey {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        json_scan::set_string(content, &self.key.refs(), &version.to_string())
            .ok_or_else(|| anyhow!("no string value at `{}`", self.raw))
    }
}
