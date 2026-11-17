//! URL helpers for compare and commit links.

/// `<repo>/compare/<prev>...<tag>`, only if both repo and previous tag exist.
pub fn compare_url(repo: Option<&str>, previous_tag: Option<&str>, tag: &str) -> Option<String> {
    match (repo, previous_tag) {
        (Some(repo), Some(prev)) => Some(format!("{repo}/compare/{prev}...{tag}")),
        _ => None,
    }
}

/// `<repo>/commit/<sha>`. Works on Gitea/Forgejo, GitHub and GitLab
/// (GitLab redirects this form to `/-/commit/`).
pub fn commit_url(repo: &str, sha: &str) -> String {
    format!("{repo}/commit/{sha}")
}

/// First seven characters of a sha (or the whole string if shorter).
pub fn short_sha(sha: &str) -> String {
    sha.chars().take(7).collect()
}
