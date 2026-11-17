//! Commit history since a revision, optionally limited by pathspecs.

use crate::{Git, RawCommit};

/// Record separator (after each commit) and field separator (sha | message).
const RECORD_END: char = '\x1e';
const FIELD_SEP: char = '\0';

pub(crate) fn commits_since(
    git: &Git,
    since: Option<&str>,
    path: &str,
    exclude: &[String],
) -> anyhow::Result<Vec<RawCommit>> {
    let range = match since {
        Some(rev) => format!("{rev}..HEAD"),
        None => "HEAD".to_string(),
    };
    let mut args = vec!["log", "--format=%H%x00%B%x1e", range.as_str()];
    let pathspecs = pathspecs(path, exclude);
    if !pathspecs.is_empty() {
        args.push("--");
        args.extend(pathspecs.iter().map(String::as_str));
    }
    let out = git.run(&args)?;
    Ok(parse_log(&out))
}

/// Pathspec list: the path (unless "."), then `:(exclude)` entries.
fn pathspecs(path: &str, exclude: &[String]) -> Vec<String> {
    let mut specs = Vec::new();
    if !path.is_empty() && path != "." {
        specs.push(path.to_string());
    }
    specs.extend(exclude.iter().map(|e| format!(":(exclude){e}")));
    specs
}

fn parse_log(out: &str) -> Vec<RawCommit> {
    out.split(RECORD_END)
        .filter_map(|record| {
            let (sha, message) = record.trim_start_matches('\n').split_once(FIELD_SEP)?;
            Some(RawCommit {
                sha: sha.trim().to_string(),
                message: message.trim().to_string(),
            })
        })
        .collect()
}
