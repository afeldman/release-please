//! Reading the body of one version's entry (release notes).

use semver::Version;

/// Body of the entry for `version`, without its `## ` heading line, up to
/// the next `## ` heading. Accepts `## [1.2.0] ...` and `## 1.2.0 ...`
/// (optionally with a leading `v`). Returns `None` if no entry matches.
pub fn extract(changelog: &str, version: &Version) -> Option<String> {
    let mut body: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in changelog.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            if inside {
                break;
            }
            inside = heading_version(rest).is_some_and(|v| &v == version);
            continue;
        }
        if inside {
            body.push(line);
        }
    }
    inside.then(|| body.join("\n").trim().to_string())
}

/// Version from the text after `## `: `[1.2.0](...)`, `[1.2.0]`, `1.2.0 (date)`.
fn heading_version(rest: &str) -> Option<Version> {
    let token = match rest.strip_prefix('[') {
        Some(inner) => inner.split(']').next()?,
        None => rest.split_whitespace().next()?,
    };
    Version::parse(token.trim_start_matches('v')).ok()
}
