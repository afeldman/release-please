//! Git access through the `git` binary (no libgit2): simple, honours the
//! user's config (signing, credentials, hooks).

mod exec;
mod history;
mod refs;
mod url;
mod write;

#[cfg(test)]
mod tests_history;
#[cfg(test)]
mod tests_refs;
#[cfg(test)]
mod tests_write;
#[cfg(test)]
mod testutil;

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
    pub fn tags(&self) -> anyhow::Result<Vec<String>> {
        refs::tags(self)
    }
    /// Commits in `since..HEAD` (all history if `None`), newest first,
    /// limited to `path` unless it is "."; `exclude` paths are pathspec-excluded.
    pub fn commits_since(
        &self,
        since: Option<&str>,
        path: &str,
        exclude: &[String],
    ) -> anyhow::Result<Vec<RawCommit>> {
        history::commits_since(self, since, path, exclude)
    }
    pub fn head_sha(&self) -> anyhow::Result<String> {
        refs::head_sha(self)
    }
    /// Content of `path` at `rev`, or `None` if the path does not exist there.
    pub fn read_file_at(&self, rev: &str, path: &str) -> anyhow::Result<Option<String>> {
        refs::read_file_at(self, rev, path)
    }
    /// `git checkout -B branch` from current HEAD.
    pub fn checkout_new_branch(&self, branch: &str) -> anyhow::Result<()> {
        refs::checkout_new_branch(self, branch)
    }
    /// Stage given paths and commit; `sign` adds `-S`.
    pub fn commit(&self, paths: &[String], message: &str, sign: bool) -> anyhow::Result<String> {
        write::commit(self, paths, message, sign)
    }
    /// `git push --force-with-lease remote branch` (release branch is bot-owned).
    pub fn push_branch(&self, remote: &str, branch: &str) -> anyhow::Result<()> {
        write::push_branch(self, remote, branch)
    }
    /// Annotated tag (signed if `sign`) at `sha`, then push it.
    pub fn tag_and_push(
        &self,
        remote: &str,
        tag: &str,
        sha: &str,
        message: &str,
        sign: bool,
    ) -> anyhow::Result<()> {
        write::tag_and_push(self, remote, tag, sha, message, sign)
    }
    /// `origin` URL normalized to https web URL (for changelog links).
    pub fn remote_web_url(&self, remote: &str) -> anyhow::Result<Option<String>> {
        let raw = self.run(&["remote", "get-url", remote])?;
        Ok(url::to_web_url(raw.trim()))
    }
}
