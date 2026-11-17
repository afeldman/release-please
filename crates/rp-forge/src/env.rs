//! Build a [`ForgeConfig`] from CLI values or CI environment.
//! Precedence: explicit values > RP_FORGE_* > Woodpecker (CI_FORGE_TYPE,
//! CI_FORGE_URL, CI_REPO) > GitHub Actions (GITHUB_API_URL,
//! GITHUB_REPOSITORY, GITHUB_TOKEN) > GitLab CI (CI_API_V4_URL,
//! CI_PROJECT_PATH, CI_JOB_TOKEN is not enough: needs RP_FORGE_TOKEN).
//! Forgejo is treated as Gitea.
//!
//! A CI default is only used when its forge kind matches the resolved kind,
//! so `--forge github` on Woodpecker does not pick up `CI_FORGE_URL`.

mod ci;
#[cfg(test)]
mod tests;

use crate::{ForgeConfig, ForgeKind};
use anyhow::{Result, anyhow, bail};

pub fn parse_kind(s: &str) -> Result<ForgeKind> {
    match s.trim().to_ascii_lowercase().as_str() {
        "gitea" | "forgejo" => Ok(ForgeKind::Gitea),
        "github" => Ok(ForgeKind::Github),
        "gitlab" => Ok(ForgeKind::Gitlab),
        other => bail!("unknown forge kind `{other}` (expected gitea, forgejo, github or gitlab)"),
    }
}

/// `lookup` abstracts std::env::var for tests.
pub fn resolve(
    kind: Option<&str>,
    url: Option<&str>,
    repo: Option<&str>,
    token: Option<&str>,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> Result<ForgeConfig> {
    // Empty and whitespace-only values count as unset.
    let env = |key: &str| {
        lookup(key)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
    };
    let arg = |v: Option<&str>| v.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty());

    let detected = ci::detect(&env)?;
    let kind = match arg(kind).or_else(|| env("RP_FORGE_KIND")) {
        Some(k) => parse_kind(&k)?,
        None => detected
            .as_ref()
            .map(|c| c.kind)
            .ok_or_else(|| anyhow!("forge kind not set: pass --forge or set RP_FORGE_KIND"))?,
    };
    let ci = detected.filter(|c| c.kind == kind);

    let url = first([
        arg(url),
        env("RP_FORGE_URL"),
        ci.as_ref().and_then(|c| c.url.clone()),
    ])
    .ok_or_else(|| anyhow!("forge url not set: pass --forge-url or set RP_FORGE_URL"))?;
    let repo = first([
        arg(repo),
        env("RP_FORGE_REPO"),
        ci.as_ref().and_then(|c| c.repo.clone()),
    ])
    .ok_or_else(|| anyhow!("forge repo not set: pass --repo or set RP_FORGE_REPO"))?;
    let token = first([
        arg(token),
        env("RP_FORGE_TOKEN"),
        ci.as_ref().and_then(|c| c.token.clone()),
    ])
    .ok_or_else(|| anyhow!("forge token not set: set RP_FORGE_TOKEN (CI_JOB_TOKEN is not used)"))?;

    Ok(ForgeConfig {
        kind,
        url,
        repo,
        token,
    })
}

fn first<const N: usize>(values: [Option<String>; N]) -> Option<String> {
    values.into_iter().flatten().next()
}
