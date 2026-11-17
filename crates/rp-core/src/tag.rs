//! Tag names: `[component][separator]v1.2.3`.
//!
//! Defaults follow release-please: `v` prefix on, `-` separator, component
//! in the tag for every non-root package (root `.` gets no component unless
//! `include_component_in_tag` is set explicitly).

use crate::config::PackageConfig;
use semver::Version;

#[cfg(test)]
mod tests;

const DEFAULT_SEPARATOR: &str = "-";

/// Component used in tags: explicit `component`, else `package_name`, else
/// the last path segment. The root package (`.`) has none by default.
pub fn component_name(cfg: &PackageConfig) -> Option<String> {
    if let Some(c) = cfg.component.as_deref().filter(|c| !c.is_empty()) {
        return Some(c.to_string());
    }
    if is_root(cfg) {
        return None;
    }
    cfg.package_name
        .as_deref()
        .filter(|n| !n.is_empty())
        .map(str::to_string)
        .or_else(|| {
            cfg.path
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
}

/// Build the tag for a package version, e.g. `v1.2.3` or `titan-v0.2.0`.
/// Component is included when `include_component_in_tag` and a component
/// is set; `include_v_in_tag` (default true) controls the `v`.
pub fn tag_name(cfg: &PackageConfig, version: &Version) -> String {
    let v = if include_v(cfg) { "v" } else { "" };
    let ver = format!("{v}{version}");
    match component_for_tag(cfg) {
        Some(component) => format!("{component}{}{ver}", separator(cfg)),
        None => ver,
    }
}

/// Parse a tag back into a version if it belongs to this package.
///
/// The `v` prefix is optional on input. A tag with a component prefix only
/// matches when the package includes its component in tags.
pub fn parse_tag(cfg: &PackageConfig, tag: &str) -> Option<Version> {
    let rest = match component_for_tag(cfg) {
        Some(component) => tag.strip_prefix(&format!("{component}{}", separator(cfg)))?,
        None => tag,
    };
    let bare = rest.strip_prefix('v').unwrap_or(rest);
    Version::parse(bare).ok()
}

fn is_root(cfg: &PackageConfig) -> bool {
    matches!(cfg.path.trim_end_matches('/'), "" | ".")
}

fn include_v(cfg: &PackageConfig) -> bool {
    cfg.include_v_in_tag.unwrap_or(true)
}

fn separator(cfg: &PackageConfig) -> &str {
    cfg.tag_separator.as_deref().unwrap_or(DEFAULT_SEPARATOR)
}

/// The component if it goes into tags, else `None`.
fn component_for_tag(cfg: &PackageConfig) -> Option<String> {
    let include = cfg.include_component_in_tag.unwrap_or(!is_root(cfg));
    if include { component_name(cfg) } else { None }
}
