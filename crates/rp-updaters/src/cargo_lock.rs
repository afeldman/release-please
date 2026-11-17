//! `Cargo.lock`: `version` of the `[[package]]` entries with a listed name.
//!
//! Line-based: only the `version = "..."` line of matching blocks changes.
//! An empty name list makes this updater a no-op.

use crate::Updater;
use anyhow::Result;
use semver::Version;

#[derive(Debug, Clone, Default)]
pub struct CargoLock {
    names: Vec<String>,
}

impl CargoLock {
    pub fn new<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}

/// One `[[package]]` block: its name and the line index of its `version`.
struct Block {
    name: Option<String>,
    version_line: Option<usize>,
}

impl Updater for CargoLock {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        if self.names.is_empty() {
            return Ok(content.to_string());
        }
        let lines: Vec<&str> = content.split_inclusive('\n').collect();
        let edit: Vec<usize> = blocks(&lines)
            .into_iter()
            .filter(|b| b.name.as_ref().is_some_and(|n| self.names.contains(n)))
            .filter_map(|b| b.version_line)
            .collect();

        let mut out = String::with_capacity(content.len());
        for (idx, line) in lines.iter().enumerate() {
            if edit.contains(&idx) {
                out.push_str(&version_line(line, version));
            } else {
                out.push_str(line);
            }
        }
        Ok(out)
    }
}

fn blocks(lines: &[&str]) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut in_package = false;
    for (idx, line) in lines.iter().enumerate() {
        let body = line.trim_end_matches(['\r', '\n']);
        if body == "[[package]]" {
            out.push(Block {
                name: None,
                version_line: None,
            });
            in_package = true;
            continue;
        }
        if body.starts_with('[') {
            in_package = false;
            continue;
        }
        if !in_package {
            continue;
        }
        if let Some(block) = out.last_mut() {
            if block.name.is_none() {
                block.name = quoted_value(body, "name");
            }
            if block.version_line.is_none() && quoted_value(body, "version").is_some() {
                block.version_line = Some(idx);
            }
        }
    }
    out
}

/// Value of a top-level `key = "value"` line.
fn quoted_value(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.strip_prefix(" = \"")?;
    rest.strip_suffix('"').map(str::to_string)
}

/// Rebuilds a version line, keeping its original line ending.
fn version_line(line: &str, version: &Version) -> String {
    let body = line.trim_end_matches(['\r', '\n']);
    let ending = &line[body.len()..];
    format!("version = \"{version}\"{ending}")
}
