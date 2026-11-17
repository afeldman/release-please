//! Gitlab REST client (ureq, blocking).

use crate::{Forge, ForgeConfig, NewRelease, PullRequest};

pub struct Gitlab {
    cfg: ForgeConfig,
    agent: ureq::Agent,
}

impl Gitlab {
    pub fn new(cfg: ForgeConfig) -> Self {
        Self { cfg, agent: ureq::Agent::new() }
    }
}

impl Forge for Gitlab {
    fn find_open_pr(&self, head: &str, base: &str) -> anyhow::Result<Option<PullRequest>> { let _ = (&self.cfg, &self.agent, head, base); todo!() }
    fn create_pr(&self, head: &str, base: &str, title: &str, body: &str) -> anyhow::Result<PullRequest> { let _ = (head, base, title, body); todo!() }
    fn update_pr(&self, number: u64, title: &str, body: &str) -> anyhow::Result<()> { let _ = (number, title, body); todo!() }
    fn add_labels(&self, number: u64, labels: &[&str]) -> anyhow::Result<()> { let _ = (number, labels); todo!() }
    fn remove_label(&self, number: u64, label: &str) -> anyhow::Result<()> { let _ = (number, label); todo!() }
    fn merged_prs_with_label(&self, base: &str, label: &str) -> anyhow::Result<Vec<PullRequest>> { let _ = (base, label); todo!() }
    fn create_release(&self, release: &NewRelease) -> anyhow::Result<()> { let _ = release; todo!() }
    fn release_exists(&self, tag: &str) -> anyhow::Result<bool> { let _ = tag; todo!() }
}
