//! Build a [`ForgeConfig`] from CLI values or CI environment.
//! Precedence: explicit values > RP_FORGE_* > Woodpecker (CI_FORGE_TYPE,
//! CI_FORGE_URL, CI_REPO) > GitHub Actions (GITHUB_API_URL,
//! GITHUB_REPOSITORY, GITHUB_TOKEN) > GitLab CI (CI_API_V4_URL,
//! CI_PROJECT_PATH, CI_JOB_TOKEN is not enough: needs RP_FORGE_TOKEN).
//! Forgejo is treated as Gitea.

use crate::{ForgeConfig, ForgeKind};

pub fn parse_kind(s: &str) -> anyhow::Result<ForgeKind> {
    let _ = s;
    todo!()
}

/// `lookup` abstracts std::env::var for tests.
pub fn resolve(
    kind: Option<&str>,
    url: Option<&str>,
    repo: Option<&str>,
    token: Option<&str>,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> anyhow::Result<ForgeConfig> {
    let _ = (kind, url, repo, token, lookup);
    todo!()
}
