//! CI environment defaults: Woodpecker, GitHub Actions, GitLab CI.

use super::parse_kind;
use crate::ForgeKind;
use anyhow::Result;

const GITHUB_DEFAULT_API: &str = "https://api.github.com";

/// Values a CI platform provides for the forge it runs against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CiDefaults {
    pub kind: ForgeKind,
    pub url: Option<String>,
    pub repo: Option<String>,
    pub token: Option<String>,
}

/// First CI platform detected wins: Woodpecker, then GitHub Actions, then GitLab CI.
pub(super) fn detect(get: &dyn Fn(&str) -> Option<String>) -> Result<Option<CiDefaults>> {
    if let Some(forge_type) = get("CI_FORGE_TYPE") {
        return Ok(Some(CiDefaults {
            kind: parse_kind(&forge_type)?,
            url: get("CI_FORGE_URL"),
            repo: get("CI_REPO"),
            token: None,
        }));
    }
    if get("GITHUB_REPOSITORY").is_some() {
        return Ok(Some(CiDefaults {
            kind: ForgeKind::Github,
            url: Some(get("GITHUB_API_URL").unwrap_or_else(|| GITHUB_DEFAULT_API.to_owned())),
            repo: get("GITHUB_REPOSITORY"),
            token: get("GITHUB_TOKEN"),
        }));
    }
    if get("CI_PROJECT_PATH").is_some() {
        return Ok(Some(CiDefaults {
            kind: ForgeKind::Gitlab,
            url: get("CI_API_V4_URL").map(|u| strip_api_v4(&u)),
            repo: get("CI_PROJECT_PATH"),
            token: None,
        }));
    }
    Ok(None)
}

/// `https://gitlab.com/api/v4` -> `https://gitlab.com` (web root).
fn strip_api_v4(url: &str) -> String {
    url.trim_end_matches('/')
        .trim_end_matches("/api/v4")
        .to_owned()
}
