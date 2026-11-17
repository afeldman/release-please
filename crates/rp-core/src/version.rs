//! Version bump rules (semver, release-please semantics).

mod core_bump;
mod prerelease;
#[cfg(test)]
mod tests;

use crate::commit::{self, ParsedCommit};
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
    commits.iter().map(commit_bump).max().unwrap_or(Bump::None)
}

fn commit_bump(commit: &ParsedCommit) -> Bump {
    if commit.breaking {
        return Bump::Major;
    }
    match commit.kind.to_ascii_lowercase().as_str() {
        "feat" => Bump::Minor,
        "fix" | "perf" | "revert" => Bump::Patch,
        _ => Bump::None,
    }
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
    if let Some(forced) = overridden_version(commits, cfg) {
        return Some(forced);
    }
    let bump = bump_for(commits);
    if bump == Bump::None {
        return None;
    }
    match previous {
        None => prerelease::initial(cfg),
        Some(prev) => prerelease::next_from(prev, bump, cfg),
    }
}

fn overridden_version(commits: &[ParsedCommit], cfg: &PackageConfig) -> Option<Version> {
    cfg.release_as
        .as_deref()
        .and_then(commit::parse_version_lenient)
        .or_else(|| commit::release_as(commits))
}
