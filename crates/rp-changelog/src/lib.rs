//! Markdown changelog entries (release-please style), templatable.

use rp_core::{ChangelogSection, ParsedCommit};
use semver::Version;

pub struct EntryInput<'a> {
    pub version: &'a Version,
    pub previous_tag: Option<&'a str>,
    pub tag: &'a str,
    /// ISO date, e.g. "2026-11-17".
    pub date: &'a str,
    pub commits: &'a [ParsedCommit],
    pub sections: &'a [ChangelogSection],
    /// e.g. "https://git.example/org/repo"; used for compare and commit links.
    pub repo_url: Option<&'a str>,
    /// Optional minijinja template replacing the built-in one.
    pub template: Option<&'a str>,
}

/// Render one entry: `## [1.2.0](compare) (date)`, then `### Section`
/// headings in section order, breaking changes first under
/// "⚠ BREAKING CHANGES", bullets `* **scope:** description ([sha7](link))`.
pub fn render_entry(input: &EntryInput) -> anyhow::Result<String> {
    let _ = input;
    todo!()
}

/// Insert `entry` after the `# Changelog` header (create header if missing).
pub fn prepend(existing: &str, entry: &str) -> String {
    let _ = (existing, entry);
    todo!()
}

/// Extract the body of the entry for `version` (used as release notes).
pub fn extract(changelog: &str, version: &Version) -> Option<String> {
    let _ = (changelog, version);
    todo!()
}
