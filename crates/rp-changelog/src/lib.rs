//! Markdown changelog entries (release-please style), templatable.
//!
//! Built-in layout:
//!
//! ```text
//! ## [1.2.0](<repo>/compare/<prev>...<tag>) (2026-11-17)
//!
//! ### ⚠ BREAKING CHANGES
//!
//! * **scope:** breaking description
//!
//! ### Features
//!
//! * **scope:** description ([abc1234](<repo>/commit/<sha>))
//! ```
//!
//! Without `repo_url` or without a previous tag the heading has no link and
//! commit bullets have no link (`(abc1234)`). Sections keep the configured
//! order; hidden sections and commits of unknown types are skipped.
//! Commit links always use `/commit/<sha>`; GitLab redirects that form to
//! its `/-/commit/` page.

mod default_render;
mod extract;
mod links;
mod model;
mod prepend;
mod template;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_file;
#[cfg(test)]
mod tests_template;

pub use extract::extract;
pub use model::{BreakingView, CommitView, SectionView, TemplateContext};
pub use prepend::prepend;

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
    /// Context variables: see [`TemplateContext`].
    pub template: Option<&'a str>,
}

/// Render one entry: `## [1.2.0](compare) (date)`, then `### Section`
/// headings in section order, breaking changes first under
/// "⚠ BREAKING CHANGES", bullets `* **scope:** description ([sha7](link))`.
///
/// The result starts with `## ` and ends with exactly one newline.
pub fn render_entry(input: &EntryInput) -> anyhow::Result<String> {
    let ctx = model::build_context(input);
    let body = match input.template {
        Some(source) => template::render(source, &ctx)?,
        None => default_render::render(&ctx),
    };
    Ok(format!("{}\n", body.trim_end()))
}
