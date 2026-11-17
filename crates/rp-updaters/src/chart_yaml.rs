//! Helm `Chart.yaml`: top-level `version` (required) and `appVersion`
//! (updated when present). Quotes and comments on those lines are kept.

use crate::Updater;
use crate::yaml_scan;
use anyhow::{Result, anyhow};
use semver::Version;

#[derive(Debug, Clone, Copy, Default)]
pub struct ChartYaml;

impl Updater for ChartYaml {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let v = version.to_string();
        let out = yaml_scan::set_scalar(content, &["version"], &v)
            .ok_or_else(|| anyhow!("Chart.yaml has no top-level version scalar"))?;
        Ok(yaml_scan::set_scalar(&out, &["appVersion"], &v).unwrap_or(out))
    }
}
