//! GitHub REST client (ureq, blocking).
//! `ForgeConfig.url` is the API root (default `https://api.github.com`;
//! GitHub Enterprise Server: `https://ghe.example/api/v3`).

mod labels;
mod pulls;
#[cfg(test)]
mod tests;

use crate::encode;
use crate::http::Api;
use crate::{Forge, ForgeConfig, NewRelease, PullRequest, rest_release};

pub struct Github {
    api: Api,
    /// Percent-encoded `owner/name`.
    repo: String,
    /// Repository owner, used for the `head=owner:branch` filter.
    owner: String,
}

impl Github {
    pub fn new(cfg: ForgeConfig) -> Self {
        let headers = vec![
            ("Authorization", format!("Bearer {}", cfg.token)),
            ("Accept", "application/vnd.github+json".to_owned()),
            ("X-GitHub-Api-Version", "2022-11-28".to_owned()),
            (
                "User-Agent",
                format!("rp-forge/{}", env!("CARGO_PKG_VERSION")),
            ),
        ];
        let owner = cfg.repo.split('/').next().unwrap_or_default().to_owned();
        Self {
            api: Api::new(&cfg.url, &cfg.token, headers),
            repo: encode::path(&cfg.repo),
            owner,
        }
    }

    fn repo_path(&self) -> String {
        format!("/repos/{}", self.repo)
    }
}

impl Forge for Github {
    fn find_open_pr(&self, head: &str, base: &str) -> anyhow::Result<Option<PullRequest>> {
        pulls::find_open_pr(self, head, base)
    }

    fn create_pr(
        &self,
        head: &str,
        base: &str,
        title: &str,
        body: &str,
    ) -> anyhow::Result<PullRequest> {
        pulls::create_pr(self, head, base, title, body)
    }

    fn update_pr(&self, number: u64, title: &str, body: &str) -> anyhow::Result<()> {
        pulls::update_pr(self, number, title, body)
    }

    fn add_labels(&self, number: u64, labels: &[&str]) -> anyhow::Result<()> {
        labels::add_labels(self, number, labels)
    }

    fn remove_label(&self, number: u64, label: &str) -> anyhow::Result<()> {
        labels::remove_label(self, number, label)
    }

    fn merged_prs_with_label(&self, base: &str, label: &str) -> anyhow::Result<Vec<PullRequest>> {
        pulls::merged_prs_with_label(self, base, label)
    }

    fn create_release(&self, release: &NewRelease) -> anyhow::Result<()> {
        rest_release::create(&self.api, &self.repo_path(), release)
    }

    fn release_exists(&self, tag: &str) -> anyhow::Result<bool> {
        rest_release::exists(&self.api, &self.repo_path(), tag)
    }
}
