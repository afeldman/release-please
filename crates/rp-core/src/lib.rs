//! Core model: configuration, parsed commits, version bumps and tag names.
//!
//! The config format is a compatible subset of release-please's
//! `release-please-config.json` + `.release-please-manifest.json`.

pub mod commit;
pub mod config;
pub mod manifest;
pub mod tag;
pub mod version;

pub use commit::ParsedCommit;
pub use config::{ChangelogSection, Config, ExtraFile, PackageConfig, ReleaseType};
pub use manifest::Manifest;
pub use version::Bump;
