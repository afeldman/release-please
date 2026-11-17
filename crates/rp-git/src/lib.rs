//! Git access through the `git` binary (no libgit2): simple, honours the
//! user's config (signing, credentials, hooks).

use std::path::PathBuf;

pub struct Git {
    pub dir: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawCommit {
    pub sha: String,
    pub message: String,
}

impl Git {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }
    /// All tags (`git tag --list`).
    pub fn tags(&self) -> anyhow::Result<Vec<String>> { todo!() }
    /// Commits in `since..HEAD` (all history if `None`), newest first,
    /// limited to `path` unless it is "."; `exclude` paths are pathspec-excluded.
    pub fn commits_since(&self, since: Option<&str>, path: &str, exclude: &[String]) -> anyhow::Result<Vec<RawCommit>> {
        let _ = (since, path, exclude);
        todo!()
    }
    pub fn head_sha(&self) -> anyhow::Result<String> { todo!() }
    pub fn read_file_at(&self, rev: &str, path: &str) -> anyhow::Result<Option<String>> {
        let _ = (rev, path);
        todo!()
    }
    /// `git checkout -B branch` from current HEAD.
    pub fn checkout_new_branch(&self, branch: &str) -> anyhow::Result<()> { let _ = branch; todo!() }
    /// Stage given paths and commit; `sign` adds `-S`.
    pub fn commit(&self, paths: &[String], message: &str, sign: bool) -> anyhow::Result<String> {
        let _ = (paths, message, sign);
        todo!()
    }
    /// `git push --force-with-lease remote branch` (release branch is bot-owned).
    pub fn push_branch(&self, remote: &str, branch: &str) -> anyhow::Result<()> { let _ = (remote, branch); todo!() }
    /// Annotated tag (signed if `sign`) at `sha`, then push it.
    pub fn tag_and_push(&self, remote: &str, tag: &str, sha: &str, message: &str, sign: bool) -> anyhow::Result<()> {
        let _ = (remote, tag, sha, message, sign);
        todo!()
    }
    /// `origin` URL normalized to https web URL (for changelog links).
    pub fn remote_web_url(&self, remote: &str) -> anyhow::Result<Option<String>> { let _ = remote; todo!() }
}
