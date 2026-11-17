//! Running `git -C <dir> ...` and turning failures into errors with stderr.

use crate::Git;
use anyhow::{Context as _, bail};
use std::process::{Command, Output, Stdio};

impl Git {
    /// Run git and return stdout as text. Fails with stderr on non-zero exit.
    pub(crate) fn run(&self, args: &[&str]) -> anyhow::Result<String> {
        let out = self.output(args)?;
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    /// Run git and return the raw process output (no error on non-zero exit).
    pub(crate) fn output(&self, args: &[&str]) -> anyhow::Result<Output> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.dir)
            .args(args)
            .stdin(Stdio::null())
            .output()
            .with_context(|| format!("failed to start git {}", args.join(" ")))?;
        if !out.status.success() {
            bail!(
                "git {} failed ({}): {}",
                args.join(" "),
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Ok(out)
    }

    /// Run git and report only whether it exited successfully.
    pub(crate) fn succeeds(&self, args: &[&str]) -> anyhow::Result<bool> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.dir)
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .with_context(|| format!("failed to start git {}", args.join(" ")))?;
        Ok(out.status.success())
    }
}
