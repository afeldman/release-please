//! Conventional-commit parsing.

use git_conventional::Commit;

#[cfg(test)]
mod tests;

/// A conventional commit. Non-conventional commits are dropped by [`parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommit {
    pub sha: String,
    pub kind: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking: bool,
    /// Text of a `BREAKING CHANGE:` footer, or the description for `type!:`.
    pub breaking_description: Option<String>,
    /// Footers as (token, value), e.g. ("Release-As", "2.0.0").
    pub footers: Vec<(String, String)>,
}

/// Parse one commit message. Returns `None` if it is not conventional.
pub fn parse(sha: &str, message: &str) -> Option<ParsedCommit> {
    let commit = Commit::parse(message.trim_end()).ok()?;
    let footers: Vec<(String, String)> = commit
        .footers()
        .iter()
        .map(|f| (f.token().as_str().to_string(), f.value().to_string()))
        .collect();
    let breaking_footer = commit
        .footers()
        .iter()
        .find(|f| f.breaking())
        .map(|f| f.value().to_string());
    let breaking = commit.breaking() || breaking_footer.is_some();
    let breaking_description =
        breaking_footer.or_else(|| commit.breaking().then(|| commit.description().to_string()));
    Some(ParsedCommit {
        sha: sha.to_string(),
        kind: commit.type_().as_str().to_lowercase(),
        scope: commit.scope().map(|s| s.as_str().to_string()),
        description: commit.description().to_string(),
        body: commit.body().map(str::to_string),
        breaking,
        breaking_description,
        footers,
    })
}

/// Value of a `Release-As:` footer, newest commit wins.
///
/// Commits are expected newest first (git log order, as returned by
/// `rp-git`). The token is matched case-insensitively. Unparsable values
/// are skipped.
pub fn release_as(commits: &[ParsedCommit]) -> Option<semver::Version> {
    commits
        .iter()
        .flat_map(|c| c.footers.iter())
        .filter(|(token, _)| token.eq_ignore_ascii_case("release-as"))
        .filter_map(|(_, value)| parse_version_lenient(value))
        .next()
}

/// Parse a version string, accepting an optional leading `v`.
pub(crate) fn parse_version_lenient(value: &str) -> Option<semver::Version> {
    semver::Version::parse(value.trim().trim_start_matches(['v', 'V'])).ok()
}
