//! Core (major.minor.patch) arithmetic: pre-major adjustments and bumps.

use super::Bump;
use crate::config::PackageConfig;
use semver::Version;

/// Apply the pre-1.0 demotions from the config. `prev` is the current
/// version; only versions with major 0 are affected.
pub(super) fn effective(bump: Bump, prev: &Version, cfg: &PackageConfig) -> Bump {
    if prev.major != 0 {
        return bump;
    }
    match bump {
        Bump::Major if cfg.bump_minor_pre_major => Bump::Minor,
        Bump::Minor if cfg.bump_patch_for_minor_pre_major => Bump::Patch,
        other => other,
    }
}

/// Bump the core of `core` (pre-release and build metadata are dropped).
pub(super) fn apply(core: &Version, bump: Bump) -> Version {
    match bump {
        Bump::Major => Version::new(core.major + 1, 0, 0),
        Bump::Minor => Version::new(core.major, core.minor + 1, 0),
        Bump::Patch | Bump::None => Version::new(core.major, core.minor, core.patch + 1),
    }
}

/// True if a bump of this size is already covered by a pre-release whose
/// core is `prev`: a major bump needs `X.0.0`, a minor bump needs `X.Y.0`,
/// a patch bump always fits.
pub(super) fn keeps_core(prev: &Version, bump: Bump) -> bool {
    match bump {
        Bump::Major => prev.minor == 0 && prev.patch == 0,
        Bump::Minor => prev.patch == 0,
        Bump::Patch | Bump::None => true,
    }
}
