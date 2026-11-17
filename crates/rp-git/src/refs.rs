//! Tags, HEAD, branches and file contents at a revision.

use crate::Git;
use anyhow::bail;

pub(crate) fn tags(git: &Git) -> anyhow::Result<Vec<String>> {
    let out = git.run(&["tag", "--list"])?;
    Ok(out
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect())
}

pub(crate) fn head_sha(git: &Git) -> anyhow::Result<String> {
    Ok(git.run(&["rev-parse", "HEAD"])?.trim().to_string())
}

pub(crate) fn checkout_new_branch(git: &Git, branch: &str) -> anyhow::Result<()> {
    git.run(&["checkout", "-B", branch])?;
    Ok(())
}

/// `None` if the path is absent at `rev`; an error if `rev` is invalid.
pub(crate) fn read_file_at(git: &Git, rev: &str, path: &str) -> anyhow::Result<Option<String>> {
    let spec = format!("{rev}:{path}");
    if !git.succeeds(&["cat-file", "-e", spec.as_str()])? {
        let commit = format!("{rev}^{{commit}}");
        if !git.succeeds(&["rev-parse", "--verify", "--quiet", commit.as_str()])? {
            bail!("unknown revision '{rev}'");
        }
        return Ok(None);
    }
    Ok(Some(git.run(&["show", spec.as_str()])?))
}
