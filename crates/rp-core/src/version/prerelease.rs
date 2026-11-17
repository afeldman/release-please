//! Pre-release handling (`-<type>.N`) and the initial version.
//!
//! A pre-release whose core already covers the bump keeps its core and
//! increments N (`1.2.0-rc.1` + feat -> `1.2.0-rc.2`). Otherwise the core is
//! bumped and the counter restarts at 0 (`1.2.0` + breaking -> `2.0.0-rc.0`).
//! A pre-release that is graduated (no `prerelease_type` configured) drops
//! the suffix when its core is covered (`1.2.0-rc.1` + fix -> `1.2.0`).

use super::Bump;
use super::core_bump;
use crate::commit;
use crate::config::PackageConfig;
use semver::Version;

/// Version used when there is no previous release.
pub(super) fn initial(cfg: &PackageConfig) -> Option<Version> {
    let default = if cfg.bump_minor_pre_major {
        Version::new(0, 1, 0)
    } else {
        Version::new(1, 0, 0)
    };
    let base = cfg
        .initial_version
        .as_deref()
        .and_then(commit::parse_version_lenient)
        .unwrap_or(default);
    match prerelease_type(cfg) {
        Some(t) if base.pre.is_empty() => with_pre(&base, t, 0),
        _ => Some(base),
    }
}

/// Next version after `prev` for a non-`None` bump.
pub(super) fn next_from(prev: &Version, bump: Bump, cfg: &PackageConfig) -> Option<Version> {
    let rule = core_bump::effective(bump, prev, cfg);
    let keep = !prev.pre.is_empty() && core_bump::keeps_core(prev, rule);
    let prev_core = Version::new(prev.major, prev.minor, prev.patch);
    let core = if keep {
        prev_core
    } else {
        core_bump::apply(&prev_core, rule)
    };
    match prerelease_type(cfg) {
        None => Some(core),
        Some(t) => {
            let n = if keep {
                continue_number(prev.pre.as_str(), t)
            } else {
                0
            };
            with_pre(&core, t, n)
        }
    }
}

/// Configured `prerelease_type`, trimmed; empty counts as unset.
fn prerelease_type(cfg: &PackageConfig) -> Option<&str> {
    cfg.prerelease_type
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

/// `N + 1` if `pre` is `<t>.N`, otherwise 0 (a new counter).
fn continue_number(pre: &str, t: &str) -> u64 {
    pre.strip_prefix(t)
        .and_then(|rest| rest.strip_prefix('.'))
        .and_then(|n| n.parse::<u64>().ok())
        .map_or(0, |n| n.saturating_add(1))
}

/// `core-<t>.<n>`; `None` if the type is not a valid pre-release identifier.
fn with_pre(core: &Version, t: &str, n: u64) -> Option<Version> {
    Version::parse(&format!(
        "{}.{}.{}-{t}.{n}",
        core.major, core.minor, core.patch
    ))
    .ok()
}
