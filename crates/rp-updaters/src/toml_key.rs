//! Extra-file TOML: the string at a dotted `jsonpath` (`$.` prefix optional),
//! rewritten through `toml_edit` so formatting and comments survive.

use crate::Updater;
use crate::key_path::KeyPath;
use crate::toml_util;
use anyhow::{Result, bail};
use semver::Version;

#[derive(Debug, Clone)]
pub struct TomlKey {
    key: KeyPath,
    raw: String,
}

impl TomlKey {
    pub fn new(jsonpath: &str) -> Self {
        Self {
            key: KeyPath::parse(jsonpath),
            raw: jsonpath.to_string(),
        }
    }
}

impl Updater for TomlKey {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let mut doc = toml_util::parse(content)?;
        if !toml_util::set_string_at(&mut doc, &self.key.refs(), &version.to_string()) {
            bail!("no string value at `{}`", self.raw);
        }
        Ok(doc.to_string())
    }
}
