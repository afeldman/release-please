//! `version.txt`: the whole file is the version followed by a newline.

use crate::Updater;
use anyhow::Result;
use semver::Version;

#[derive(Debug, Clone, Copy, Default)]
pub struct VersionTxt;

impl Updater for VersionTxt {
    fn update(&self, _content: &str, version: &Version) -> Result<String> {
        Ok(format!("{version}\n"))
    }
}
