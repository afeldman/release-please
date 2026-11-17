//! Template context and the mapping from commits to sections.

use crate::EntryInput;
use crate::links;
use rp_core::ParsedCommit;
use serde::Serialize;

/// Context passed to a custom minijinja template.
///
/// Variables available in the template:
/// - `version`: version string without `v`, e.g. `"1.2.0"`
/// - `tag`: tag name, e.g. `"v1.2.0"`
/// - `previous_tag`: previous tag or `none`
/// - `date`: ISO date string
/// - `repo_url`: repository web URL without trailing slash, or `none`
/// - `compare_url`: `<repo>/compare/<prev>...<tag>`, or `none`
/// - `breaking`: list of `{scope, description}` (scope may be `none`)
/// - `sections`: list of `{title, commits}`, visible sections only and
///   without empty ones; each commit is `{sha, short_sha, scope,
///   description, link}` where `link` is the commit URL or `none`.
///
/// The template renders the whole entry, including the `## ` heading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TemplateContext {
    pub version: String,
    pub tag: String,
    pub previous_tag: Option<String>,
    pub date: String,
    pub repo_url: Option<String>,
    pub compare_url: Option<String>,
    pub breaking: Vec<BreakingView>,
    pub sections: Vec<SectionView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BreakingView {
    pub scope: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SectionView {
    pub title: String,
    pub commits: Vec<CommitView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitView {
    pub sha: String,
    pub short_sha: String,
    pub scope: Option<String>,
    pub description: String,
    pub link: Option<String>,
}

pub(crate) fn build_context(input: &EntryInput) -> TemplateContext {
    let repo = input.repo_url.map(|r| r.trim_end_matches('/').to_string());
    let previous_tag = input.previous_tag.map(str::to_string);
    TemplateContext {
        version: input.version.to_string(),
        tag: input.tag.to_string(),
        date: input.date.to_string(),
        compare_url: links::compare_url(repo.as_deref(), input.previous_tag, input.tag),
        breaking: breaking_views(input.commits),
        sections: section_views(input, repo.as_deref()),
        previous_tag,
        repo_url: repo,
    }
}

fn breaking_views(commits: &[ParsedCommit]) -> Vec<BreakingView> {
    commits
        .iter()
        .filter(|c| c.breaking)
        .map(|c| BreakingView {
            scope: c.scope.clone(),
            description: c
                .breaking_description
                .clone()
                .unwrap_or_else(|| c.description.clone()),
        })
        .collect()
}

/// Visible sections in configured order; commits of matching type only.
fn section_views(input: &EntryInput, repo: Option<&str>) -> Vec<SectionView> {
    input
        .sections
        .iter()
        .filter(|s| !s.hidden)
        .filter_map(|s| {
            let commits: Vec<CommitView> = input
                .commits
                .iter()
                .filter(|c| c.kind == s.kind)
                .map(|c| commit_view(c, repo))
                .collect();
            (!commits.is_empty()).then(|| SectionView {
                title: s.section.clone(),
                commits,
            })
        })
        .collect()
}

fn commit_view(commit: &ParsedCommit, repo: Option<&str>) -> CommitView {
    CommitView {
        sha: commit.sha.clone(),
        short_sha: links::short_sha(&commit.sha),
        scope: commit.scope.clone(),
        description: commit.description.clone(),
        link: repo.map(|r| links::commit_url(r, &commit.sha)),
    }
}
