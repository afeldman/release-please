//! `pyproject.toml`: `project.version`, or `tool.poetry.version` as fallback.
//! Skipped when `project.dynamic` lists `version`.

use crate::Updater;
use crate::toml_util;
use anyhow::{Result, bail};
use semver::Version;
use toml_edit::{DocumentMut, Item};

#[derive(Debug, Clone, Copy, Default)]
pub struct PyProject;

impl Updater for PyProject {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let mut doc = toml_util::parse(content)?;
        if has_dynamic_version(&doc) {
            return Ok(content.to_string());
        }
        let v = version.to_string();
        if toml_util::set_string_at(&mut doc, &["project", "version"], &v)
            || toml_util::set_string_at(&mut doc, &["tool", "poetry", "version"], &v)
        {
            return Ok(doc.to_string());
        }
        bail!("pyproject.toml has no string project.version or tool.poetry.version")
    }
}

fn has_dynamic_version(doc: &DocumentMut) -> bool {
    doc.get("project")
        .and_then(Item::as_table_like)
        .and_then(|project| project.get("dynamic"))
        .and_then(Item::as_array)
        .is_some_and(|list| list.iter().any(|v| v.as_str() == Some("version")))
}
