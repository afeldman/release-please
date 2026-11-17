//! Converting git remote URLs into https web URLs.
//!
//! Handled forms: `https://[user[:pass]@]host[:port]/org/repo[.git]`,
//! `ssh://[user@]host[:port]/org/repo[.git]` and scp-like
//! `[user@]host:org/repo[.git]`. The output is always `https://host/org/repo`;
//! credentials and ssh ports are dropped (the web port is not the ssh port).

/// `None` for an unknown form or an empty path.
pub fn to_web_url(raw: &str) -> Option<String> {
    let raw = raw.trim();
    let web = raw
        .strip_prefix("https://")
        .or_else(|| raw.strip_prefix("http://"));
    let (host, path) = if let Some(rest) = web {
        let (authority, path) = rest.split_once('/')?;
        (host_with_port(authority), path)
    } else if let Some(rest) = raw.strip_prefix("ssh://") {
        let (authority, path) = rest.split_once('/')?;
        (host_without_port(authority), path)
    } else if raw.contains("://") {
        return None;
    } else {
        let (authority, path) = raw.split_once(':')?;
        (host_without_port(authority), path)
    };
    let path = clean_path(path);
    if host.is_empty() || path.is_empty() {
        return None;
    }
    Some(format!("https://{host}/{path}"))
}

/// Drop `user[:pass]@` and keep the port (web URLs may use one).
fn host_with_port(authority: &str) -> &str {
    authority.rsplit('@').next().unwrap_or(authority)
}

/// Drop `user@` and `:port` (ssh ports are not web ports).
fn host_without_port(authority: &str) -> &str {
    let host = host_with_port(authority);
    host.split(':').next().unwrap_or(host)
}

fn clean_path(path: &str) -> String {
    let path = path.trim_start_matches('/').trim_end_matches('/');
    path.strip_suffix(".git").unwrap_or(path).to_string()
}

#[cfg(test)]
mod tests {
    use super::to_web_url;

    #[test]
    fn url_conversion_table() {
        let cases = [
            (
                "git@git.example.com:org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "git@git.example.com:org/repo",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "ssh://git@git.example.com/org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "ssh://git@git.example.com:2222/org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "ssh://git.example.com/org/sub/repo",
                Some("https://git.example.com/org/sub/repo"),
            ),
            (
                "https://git.example.com/org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "https://git.example.com/org/repo",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "https://git.example.com/org/repo/",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "https://test-user:test-token@git.example.com/org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "https://git.example.com:8443/org/repo.git",
                Some("https://git.example.com:8443/org/repo"),
            ),
            (
                "http://git.example.com/org/repo.git",
                Some("https://git.example.com/org/repo"),
            ),
            (
                "  https://git.example.com/org/repo.git\n",
                Some("https://git.example.com/org/repo"),
            ),
            ("https://git.example.com/", None),
            ("file:///srv/git/repo.git", None),
            ("/local/path/repo.git", None),
            ("", None),
        ];
        for (raw, expected) in cases {
            assert_eq!(to_web_url(raw).as_deref(), expected, "input: {raw:?}");
        }
    }
}
