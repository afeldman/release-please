//! Writing: commits, branch pushes and annotated tags.

use crate::Git;

/// Stage `paths` and commit only those paths; returns the new HEAD sha.
pub(crate) fn commit(
    git: &Git,
    paths: &[String],
    message: &str,
    sign: bool,
) -> anyhow::Result<String> {
    if !paths.is_empty() {
        let mut add = vec!["add", "--"];
        add.extend(paths.iter().map(String::as_str));
        git.run(&add)?;
    }
    let mut args = vec!["commit"];
    if sign {
        args.push("-S");
    }
    args.extend(["-m", message]);
    if !paths.is_empty() {
        args.push("--");
        args.extend(paths.iter().map(String::as_str));
    }
    git.run(&args)?;
    git.head_sha()
}

pub(crate) fn push_branch(git: &Git, remote: &str, branch: &str) -> anyhow::Result<()> {
    git.run(&["push", "--force-with-lease", remote, branch])?;
    Ok(())
}

pub(crate) fn tag_and_push(
    git: &Git,
    remote: &str,
    tag: &str,
    sha: &str,
    message: &str,
    sign: bool,
) -> anyhow::Result<()> {
    let kind = if sign { "-s" } else { "-a" };
    git.run(&["tag", kind, tag, sha, "-m", message])?;
    let refspec = format!("refs/tags/{tag}");
    git.run(&["push", remote, &refspec])?;
    Ok(())
}
