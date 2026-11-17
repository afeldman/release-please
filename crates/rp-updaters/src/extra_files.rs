//! Maps configured extra files to updaters.
//!
//! Config errors (unknown type, missing `jsonpath`) do not fail here, since
//! the mapping returns no `Result`. They become an updater that errors on
//! `update`, so the release fails with a clear message.

use crate::generic::GenericMarkers;
use crate::json_key::JsonKey;
use crate::toml_key::TomlKey;
use crate::yaml_key::YamlKey;
use crate::{FileUpdate, Updater, join_path};
use anyhow::{Result, bail};
use rp_core::ExtraFile;
use semver::Version;

pub fn build(files: &[ExtraFile], pkg_path: &str) -> Vec<FileUpdate> {
    files
        .iter()
        .map(|file| file_update(file, pkg_path))
        .collect()
}

fn file_update(file: &ExtraFile, pkg_path: &str) -> FileUpdate {
    let (path, updater): (&str, Box<dyn Updater>) = match file {
        ExtraFile::Path(path) => (path, Box::new(GenericMarkers)),
        ExtraFile::Typed {
            kind,
            path,
            jsonpath,
        } => (path, typed_updater(kind, jsonpath.as_deref())),
    };
    FileUpdate {
        path: join_path(pkg_path, path),
        updater,
        optional: false,
    }
}

fn typed_updater(kind: &str, jsonpath: Option<&str>) -> Box<dyn Updater> {
    match (kind, jsonpath) {
        ("generic", _) => Box::new(GenericMarkers),
        ("json", Some(key)) => Box::new(JsonKey::new(key)),
        ("toml", Some(key)) => Box::new(TomlKey::new(key)),
        ("yaml", Some(key)) => Box::new(YamlKey::new(key)),
        ("json" | "toml" | "yaml", None) => Box::new(Invalid(format!(
            "extra file type `{kind}` needs a `jsonpath`"
        ))),
        (other, _) => Box::new(Invalid(format!("unknown extra file type `{other}`"))),
    }
}

/// Updater that always fails with a configuration message.
struct Invalid(String);

impl Updater for Invalid {
    fn update(&self, _content: &str, _version: &Version) -> Result<String> {
        bail!("{}", self.0)
    }
}
