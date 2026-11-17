//! Conventional-commit parsing.

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
    let _ = (sha, message);
    todo!("use git_conventional::Commit::parse")
}

/// Value of a `Release-As:` footer in any commit, last one wins.
pub fn release_as(commits: &[ParsedCommit]) -> Option<semver::Version> {
    let _ = commits;
    todo!()
}
