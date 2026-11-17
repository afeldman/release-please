//! GitLab REST client (ureq, blocking).
//! `ForgeConfig.url` is the web root (e.g. `https://gitlab.com`); the API
//! lives at `{url}/api/v4`. Merge requests map onto `PullRequest`
//! (iid -> number, description -> body).

mod labels;
mod mrs;
mod releases;
#[cfg(test)]
mod tests;

use crate::encode;
use crate::http::Api;
use crate::{Forge, ForgeConfig, NewRelease, PullRequest};

pub struct Gitlab {
    api: Api,
    /// Percent-encoded full project path (`/` becomes `%2F`).
    project: String,
}

impl Gitlab {
    pub fn new(cfg: ForgeConfig) -> Self {
        let base = format!("{}/api/v4", cfg.url.trim_end_matches('/'));
        let headers = vec![
            ("PRIVATE-TOKEN", cfg.token.clone()),
            ("Accept", "application/json".to_owned()),
        ];
        Self {
            api: Api::new(&base, &cfg.token, headers),
            project: encode::segment(&cfg.repo),
        }
    }

    fn project_path(&self) -> String {
        format!("/projects/{}", self.project)
    }
}

impl Forge for Gitlab {
    fn find_open_pr(&self, head: &str, base: &str) -> anyhow::Result<Option<PullRequest>> {
        mrs::find_open_pr(self, head, base)
    }

    fn create_pr(
        &self,
        head: &str,
        base: &str,
        title: &str,
        body: &str,
    ) -> anyhow::Result<PullRequest> {
        mrs::create_pr(self, head, base, title, body)
    }

    fn update_pr(&self, number: u64, title: &str, body: &str) -> anyhow::Result<()> {
        mrs::update_pr(self, number, title, body)
    }

    fn add_labels(&self, number: u64, labels: &[&str]) -> anyhow::Result<()> {
        labels::add_labels(self, number, labels)
    }

    fn remove_label(&self, number: u64, label: &str) -> anyhow::Result<()> {
        labels::remove_label(self, number, label)
    }

    fn merged_prs_with_label(&self, base: &str, label: &str) -> anyhow::Result<Vec<PullRequest>> {
        mrs::merged_prs_with_label(self, base, label)
    }

    fn create_release(&self, release: &NewRelease) -> anyhow::Result<()> {
        releases::create_release(self, release)
    }

    fn release_exists(&self, tag: &str) -> anyhow::Result<bool> {
        releases::release_exists(self, tag)
    }
}
