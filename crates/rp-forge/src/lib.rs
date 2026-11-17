//! Forge abstraction. GitLab merge requests map onto `PullRequest`.

pub mod env;
pub mod gitea;
pub mod github;
pub mod gitlab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub body: String,
    pub head_branch: String,
    pub base_branch: String,
    pub labels: Vec<String>,
    /// Merge commit sha, if merged.
    pub merge_sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRelease<'a> {
    pub tag: &'a str,
    pub target_sha: &'a str,
    pub name: &'a str,
    pub body: &'a str,
    pub draft: bool,
    pub prerelease: bool,
}

pub trait Forge {
    fn find_open_pr(&self, head: &str, base: &str) -> anyhow::Result<Option<PullRequest>>;
    fn create_pr(&self, head: &str, base: &str, title: &str, body: &str) -> anyhow::Result<PullRequest>;
    fn update_pr(&self, number: u64, title: &str, body: &str) -> anyhow::Result<()>;
    /// Create labels in the repo if missing, then attach.
    fn add_labels(&self, number: u64, labels: &[&str]) -> anyhow::Result<()>;
    fn remove_label(&self, number: u64, label: &str) -> anyhow::Result<()>;
    /// Merged PRs into `base` carrying `label`, newest first (max 50).
    fn merged_prs_with_label(&self, base: &str, label: &str) -> anyhow::Result<Vec<PullRequest>>;
    /// Create release; creates the tag server-side if it does not exist.
    fn create_release(&self, release: &NewRelease) -> anyhow::Result<()>;
    fn release_exists(&self, tag: &str) -> anyhow::Result<bool>;
}

/// Connection settings shared by all forges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeConfig {
    pub kind: ForgeKind,
    /// API base, e.g. https://git.example (Gitea), https://api.github.com,
    /// https://gitlab.com.
    pub url: String,
    /// "owner/repo" (GitLab: full project path).
    pub repo: String,
    pub token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeKind {
    Gitea,
    Github,
    Gitlab,
}

pub fn connect(cfg: ForgeConfig) -> Box<dyn Forge> {
    match cfg.kind {
        ForgeKind::Gitea => Box::new(gitea::Gitea::new(cfg)),
        ForgeKind::Github => Box::new(github::Github::new(cfg)),
        ForgeKind::Gitlab => Box::new(gitlab::Gitlab::new(cfg)),
    }
}
