//! Gitea / Forgejo REST client (ureq, blocking).
//! `ForgeConfig.url` is the web root; the API lives at `{url}/api/v1`.

mod labels;
mod pulls;
#[cfg(test)]
mod tests;

use crate::encode;
use crate::http::Api;
use crate::{Forge, ForgeConfig, NewRelease, PullRequest, rest_release};

pub struct Gitea {
    api: Api,
    /// Percent-encoded `owner/name`.
    repo: String,
}

impl Gitea {
    pub fn new(cfg: ForgeConfig) -> Self {
        let base = format!("{}/api/v1", cfg.url.trim_end_matches('/'));
        let headers = vec![
            ("Authorization", format!("token {}", cfg.token)),
            ("Accept", "application/json".to_owned()),
        ];
        Self {
            api: Api::new(&base, &cfg.token, headers),
            repo: encode::path(&cfg.repo),
        }
    }

    fn repo_path(&self) -> String {
        format!("/repos/{}", self.repo)
    }
}

impl Forge for Gitea {
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
