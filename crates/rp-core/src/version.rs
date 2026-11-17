//! Version bump rules (semver, release-please semantics).

use crate::commit::ParsedCommit;
use crate::config::PackageConfig;
use semver::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bump {
    None,
    Patch,
    Minor,
    Major,
}

/// Highest bump implied by commits: breaking -> Major, feat -> Minor,
/// fix/perf/revert -> Patch, other types -> None.
pub fn bump_for(commits: &[ParsedCommit]) -> Bump {
    let _ = commits;
    todo!()
}

/// Next version. Rules:
/// - `cfg.release_as` or a `Release-As` footer overrides everything.
/// - no previous version: `cfg.initial_version` (default 1.0.0 unless
///   `bump_minor_pre_major`, then 0.1.0).
/// - pre-1.0: Major -> Minor if `bump_minor_pre_major`;
///   Minor -> Patch if `bump_patch_for_minor_pre_major`.
/// - `prerelease_type` (e.g. "rc"): bump then append/increment `-rc.N`.
/// - Returns `None` if bump is `None` and nothing overrides.
pub fn next_version(
    previous: Option<&Version>,
    commits: &[ParsedCommit],
    cfg: &PackageConfig,
) -> Option<Version> {
    let _ = (previous, commits, cfg);
    todo!()
}
