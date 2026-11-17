//! `Cargo.toml`: `package.version`, `workspace.package.version` and the
//! `version` of path dependencies (release-please rust strategy).

use crate::Updater;
use crate::toml_util;
use anyhow::Result;
use semver::Version;
use std::collections::BTreeSet;
use toml_edit::{DocumentMut, TableLike};

const DEP_KINDS: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

/// Bumps the crate version and path dependencies that carry a `version`.
/// Items with `version.workspace = true` are left alone.
#[derive(Debug, Clone, Default)]
pub struct CargoToml {
    /// `None`: every path dependency with a `version` is bumped.
    /// `Some`: only dependencies with these names are bumped.
    only_deps: Option<BTreeSet<String>>,
}

impl CargoToml {
    /// Bumps all path dependencies that have a `version`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bumps only the path dependencies with these crate names.
    pub fn with_deps<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            only_deps: Some(names.into_iter().map(Into::into).collect()),
        }
    }

    fn bump_dep_tables(&self, root: &mut dyn TableLike, version: &str) {
        for kind in DEP_KINDS {
            if let Some(deps) = root.get_mut(kind).and_then(|item| item.as_table_like_mut()) {
                self.bump_table(deps, version);
            }
        }
    }

    fn bump_table(&self, deps: &mut dyn TableLike, version: &str) {
        let names: Vec<String> = deps.iter().map(|(key, _)| key.to_string()).collect();
        for name in names {
            if self
                .only_deps
                .as_ref()
                .is_some_and(|only| !only.contains(&name))
            {
                continue;
            }
            let Some(dep) = deps
                .get_mut(&name)
                .and_then(|item| item.as_table_like_mut())
            else {
                continue;
            };
            if dep.get("path").is_none() {
                continue;
            }
            if let Some(item) = dep.get_mut("version") {
                toml_util::set_string(item, version);
            }
        }
    }
}

impl Updater for CargoToml {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let mut doc: DocumentMut = toml_util::parse(content)?;
        let v = version.to_string();
        toml_util::set_string_at(&mut doc, &["package", "version"], &v);
        toml_util::set_string_at(&mut doc, &["workspace", "package", "version"], &v);

        if let Some(ws) = doc.get_mut("workspace").and_then(|i| i.as_table_like_mut()) {
            self.bump_dep_tables(ws, &v);
        }
        let root: &mut dyn TableLike = doc.as_table_mut();
        self.bump_dep_tables(root, &v);

        if let Some(target) = doc.get_mut("target").and_then(|i| i.as_table_like_mut()) {
            let names: Vec<String> = target.iter().map(|(key, _)| key.to_string()).collect();
            for name in names {
                if let Some(cfg) = target.get_mut(&name).and_then(|i| i.as_table_like_mut()) {
                    self.bump_dep_tables(cfg, &v);
                }
            }
        }
        Ok(doc.to_string())
    }
}
