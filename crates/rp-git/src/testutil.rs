//! Temp repositories for tests (local `git` only, no network).

use crate::Git;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Run git in `dir`, panic on failure, return trimmed stdout.
pub fn sh(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git must be installed");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Fresh repo with a local identity and signing disabled.
pub fn repo() -> (TempDir, Git) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path().to_path_buf();
    sh(&dir, &["init", "-q"]);
    sh(&dir, &["config", "user.name", "Test User"]);
    sh(&dir, &["config", "user.email", "test@example.com"]);
    sh(&dir, &["config", "commit.gpgsign", "false"]);
    sh(&dir, &["config", "tag.gpgsign", "false"]);
    (tmp, Git::new(dir))
}

/// Write `path` (relative to the repo), commit it with `msg`, return the sha.
pub fn commit_file(git: &Git, path: &str, content: &str, msg: &str) -> String {
    let full = git.dir.join(path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).expect("mkdir");
    }
    std::fs::write(&full, content).expect("write");
    sh(&git.dir, &["add", "-A"]);
    sh(&git.dir, &["commit", "-q", "-m", msg]);
    sh(&git.dir, &["rev-parse", "HEAD"])
}

/// Bare repository to act as `origin`; returns its temp guard and path.
pub fn bare_remote() -> (TempDir, PathBuf) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("remote.git");
    let status = Command::new("git")
        .args(["init", "-q", "--bare"])
        .arg(&path)
        .status()
        .expect("git must be installed");
    assert!(status.success());
    (tmp, path)
}
