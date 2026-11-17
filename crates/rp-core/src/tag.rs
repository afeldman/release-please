//! Tag names: `[component][separator]v1.2.3`.

use crate::config::PackageConfig;
use semver::Version;

/// Build the tag for a package version, e.g. `v1.2.3` or `titan-v0.2.0`.
/// Component is included when `include_component_in_tag` and a component
/// is set; `include_v_in_tag` (default true) controls the `v`.
pub fn tag_name(cfg: &PackageConfig, version: &Version) -> String {
    let _ = (cfg, version);
    todo!()
}

/// Parse a tag back into a version if it belongs to this package.
pub fn parse_tag(cfg: &PackageConfig, tag: &str) -> Option<Version> {
    let _ = (cfg, tag);
    todo!()
}
