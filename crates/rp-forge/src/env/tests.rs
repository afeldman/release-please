use super::{parse_kind, resolve};
use crate::ForgeKind;
use std::collections::HashMap;

/// Fake environment lookup.
fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let map: HashMap<String, String> = pairs
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    move |key| map.get(key).cloned()
}

#[test]
fn parse_kind_accepts_all_names_case_insensitively() {
    assert_eq!(parse_kind("gitea").unwrap(), ForgeKind::Gitea);
    assert_eq!(parse_kind("Forgejo").unwrap(), ForgeKind::Gitea);
    assert_eq!(parse_kind("GITHUB").unwrap(), ForgeKind::Github);
    assert_eq!(parse_kind(" gitlab ").unwrap(), ForgeKind::Gitlab);
}

#[test]
fn parse_kind_rejects_unknown_names() {
    let err = parse_kind("bitbucket").unwrap_err().to_string();
    assert!(err.contains("bitbucket"), "{err}");
}

#[test]
fn explicit_values_beat_rp_forge_env() {
    let lookup = env(&[
        ("RP_FORGE_KIND", "github"),
        ("RP_FORGE_URL", "https://env.example"),
        ("RP_FORGE_REPO", "env/repo"),
        ("RP_FORGE_TOKEN", "env-token"),
    ]);
    let cfg = resolve(
        Some("gitea"),
        Some("https://arg.example"),
        Some("arg/repo"),
        Some("arg-token"),
        &lookup,
    )
    .unwrap();
    assert_eq!(cfg.kind, ForgeKind::Gitea);
    assert_eq!(cfg.url, "https://arg.example");
    assert_eq!(cfg.repo, "arg/repo");
    assert_eq!(cfg.token, "arg-token");
}

#[test]
fn rp_forge_env_used_without_explicit_values() {
    let lookup = env(&[
        ("RP_FORGE_KIND", "gitlab"),
        ("RP_FORGE_URL", "https://gitlab.example"),
        ("RP_FORGE_REPO", "group/proj"),
        ("RP_FORGE_TOKEN", "env-token"),
    ]);
    let cfg = resolve(None, None, None, None, &lookup).unwrap();
    assert_eq!(cfg.kind, ForgeKind::Gitlab);
    assert_eq!(cfg.url, "https://gitlab.example");
    assert_eq!(cfg.repo, "group/proj");
    assert_eq!(cfg.token, "env-token");
}

#[test]
fn woodpecker_defaults_with_token_from_rp_forge_token() {
    let lookup = env(&[
        ("CI_FORGE_TYPE", "gitea"),
        ("CI_FORGE_URL", "https://git.example"),
        ("CI_REPO", "afe/crater"),
        ("RP_FORGE_TOKEN", "test-token"),
    ]);
    let cfg = resolve(None, None, None, None, &lookup).unwrap();
    assert_eq!(cfg.kind, ForgeKind::Gitea);
    assert_eq!(cfg.url, "https://git.example");
    assert_eq!(cfg.repo, "afe/crater");
    assert_eq!(cfg.token, "test-token");
}

#[test]
fn woodpecker_forgejo_type_maps_to_gitea() {
    let lookup = env(&[
        ("CI_FORGE_TYPE", "forgejo"),
        ("CI_FORGE_URL", "https://forgejo.example"),
        ("CI_REPO", "afe/crater"),
        ("RP_FORGE_TOKEN", "test-token"),
    ]);
    assert_eq!(
        resolve(None, None, None, None, &lookup).unwrap().kind,
        ForgeKind::Gitea
    );
}

#[test]
fn woodpecker_unknown_forge_type_is_an_error() {
    let lookup = env(&[("CI_FORGE_TYPE", "svn"), ("RP_FORGE_TOKEN", "test-token")]);
    assert!(resolve(None, None, None, None, &lookup).is_err());
}

#[test]
fn github_actions_defaults_use_github_token() {
    let lookup = env(&[
        ("GITHUB_API_URL", "https://api.github.com"),
        ("GITHUB_REPOSITORY", "afe/crater"),
        ("GITHUB_TOKEN", "gh-token"),
    ]);
    let cfg = resolve(None, None, None, None, &lookup).unwrap();
    assert_eq!(cfg.kind, ForgeKind::Github);
    assert_eq!(cfg.url, "https://api.github.com");
    assert_eq!(cfg.repo, "afe/crater");
    assert_eq!(cfg.token, "gh-token");
}

#[test]
fn github_actions_api_url_defaults_when_missing() {
    let lookup = env(&[
        ("GITHUB_REPOSITORY", "afe/crater"),
        ("GITHUB_TOKEN", "gh-token"),
    ]);
    let cfg = resolve(None, None, None, None, &lookup).unwrap();
    assert_eq!(cfg.url, "https://api.github.com");
}

#[test]
fn rp_forge_token_beats_github_token() {
    let lookup = env(&[
        ("GITHUB_REPOSITORY", "afe/crater"),
        ("GITHUB_TOKEN", "gh-token"),
        ("RP_FORGE_TOKEN", "rp-token"),
    ]);
    assert_eq!(
        resolve(None, None, None, None, &lookup).unwrap().token,
        "rp-token"
    );
}

#[test]
fn gitlab_ci_strips_api_suffix_and_ignores_job_token() {
    let lookup = env(&[
        ("CI_API_V4_URL", "https://gitlab.example/api/v4"),
        ("CI_PROJECT_PATH", "group/sub/proj"),
        ("CI_JOB_TOKEN", "secret-job-token"),
    ]);
    let err = resolve(None, None, None, None, &lookup)
        .unwrap_err()
        .to_string();
    assert!(err.contains("RP_FORGE_TOKEN"), "{err}");
    assert!(!err.contains("secret-job-token"), "{err}");

    let with_token = env(&[
        ("CI_API_V4_URL", "https://gitlab.example/api/v4"),
        ("CI_PROJECT_PATH", "group/sub/proj"),
        ("RP_FORGE_TOKEN", "test-token"),
    ]);
    let cfg = resolve(None, None, None, None, &with_token).unwrap();
    assert_eq!(cfg.kind, ForgeKind::Gitlab);
    assert_eq!(cfg.url, "https://gitlab.example");
    assert_eq!(cfg.repo, "group/sub/proj");
}

#[test]
fn explicit_kind_does_not_take_values_from_other_ci() {
    let lookup = env(&[
        ("CI_FORGE_TYPE", "gitea"),
        ("CI_FORGE_URL", "https://git.example"),
        ("CI_REPO", "afe/crater"),
        ("RP_FORGE_TOKEN", "test-token"),
    ]);
    let err = resolve(Some("github"), None, None, None, &lookup)
        .unwrap_err()
        .to_string();
    assert!(err.contains("url"), "{err}");
}

#[test]
fn missing_kind_without_ci_is_an_error() {
    let err = resolve(None, None, None, None, &env(&[]))
        .unwrap_err()
        .to_string();
    assert!(err.contains("forge kind"), "{err}");
}

#[test]
fn missing_token_error_does_not_leak_values() {
    let lookup = env(&[
        ("RP_FORGE_KIND", "gitea"),
        ("RP_FORGE_URL", "https://git.example"),
        ("RP_FORGE_REPO", "afe/crater"),
        ("CI_JOB_TOKEN", "secret-job-token"),
    ]);
    let err = resolve(None, None, None, None, &lookup)
        .unwrap_err()
        .to_string();
    assert!(err.contains("token"), "{err}");
    assert!(!err.contains("secret-job-token"), "{err}");
}

#[test]
fn blank_values_count_as_unset() {
    let lookup = env(&[
        ("RP_FORGE_KIND", "gitea"),
        ("RP_FORGE_URL", "  "),
        ("RP_FORGE_REPO", "afe/crater"),
        ("RP_FORGE_TOKEN", "test-token"),
        ("CI_FORGE_URL", "https://git.example"),
        ("CI_FORGE_TYPE", "gitea"),
    ]);
    let cfg = resolve(None, Some(""), None, None, &lookup).unwrap();
    assert_eq!(cfg.url, "https://git.example");
}
