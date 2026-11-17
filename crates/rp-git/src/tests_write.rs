use crate::testutil::{bare_remote, commit_file, repo, sh};
use std::path::Path;

fn remote_ref(bare: &Path, name: &str) -> String {
    sh(bare, &["rev-parse", "--verify", "--quiet", name])
}

#[test]
fn commit_stages_only_given_paths() {
    let (_tmp, git) = repo();
    commit_file(&git, "base.txt", "base\n", "feat: base");
    std::fs::write(git.dir.join("f.txt"), "f\n").unwrap();
    std::fs::write(git.dir.join("other.txt"), "o\n").unwrap();
    sh(&git.dir, &["add", "other.txt"]);
    let sha = git
        .commit(&["f.txt".to_string()], "chore: add f", false)
        .unwrap();
    assert_eq!(sha, git.head_sha().unwrap());
    assert_eq!(sh(&git.dir, &["log", "-1", "--format=%s"]), "chore: add f");
    // other.txt was staged but not in the commit: still staged.
    assert_eq!(
        sh(&git.dir, &["diff", "--cached", "--name-only"]),
        "other.txt"
    );
}

#[test]
fn push_branch_force_with_lease_after_amend() {
    let (_remote_tmp, remote) = bare_remote();
    let (_tmp, git) = repo();
    sh(
        &git.dir,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    commit_file(&git, "a.txt", "a\n", "feat: a");
    git.checkout_new_branch("release/v1").unwrap();
    let first = commit_file(&git, "b.txt", "b\n", "feat: b");
    git.push_branch("origin", "release/v1").unwrap();
    assert_eq!(remote_ref(&remote, "refs/heads/release/v1"), first);

    // Bot-owned branch: rewritten history must be pushed with force-with-lease.
    sh(
        &git.dir,
        &["commit", "-q", "--amend", "-m", "feat: b amended"],
    );
    let amended = git.head_sha().unwrap();
    git.push_branch("origin", "release/v1").unwrap();
    assert_eq!(remote_ref(&remote, "refs/heads/release/v1"), amended);
}

#[test]
fn tag_and_push_creates_annotated_tag_on_remote() {
    let (_remote_tmp, remote) = bare_remote();
    let (_tmp, git) = repo();
    sh(
        &git.dir,
        &["remote", "add", "origin", remote.to_str().unwrap()],
    );
    let sha = commit_file(&git, "a.txt", "a\n", "feat: a");
    git.tag_and_push("origin", "v1.0.0", &sha, "release v1.0.0", false)
        .unwrap();
    assert_eq!(sh(&remote, &["tag", "--list"]), "v1.0.0");
    assert_eq!(sh(&remote, &["cat-file", "-t", "refs/tags/v1.0.0"]), "tag");
    assert_eq!(sh(&remote, &["rev-parse", "v1.0.0^{commit}"]), sha);
}

#[test]
fn remote_web_url_reads_origin() {
    let (_tmp, git) = repo();
    sh(
        &git.dir,
        &[
            "remote",
            "add",
            "origin",
            "git@git.example.com:org/repo.git",
        ],
    );
    assert_eq!(
        git.remote_web_url("origin").unwrap().as_deref(),
        Some("https://git.example.com/org/repo")
    );
    sh(
        &git.dir,
        &[
            "remote",
            "set-url",
            "origin",
            "https://test-user:test-token@git.example.com/org/repo.git",
        ],
    );
    assert_eq!(
        git.remote_web_url("origin").unwrap().as_deref(),
        Some("https://git.example.com/org/repo")
    );
}

#[test]
fn remote_web_url_missing_remote_is_error() {
    let (_tmp, git) = repo();
    assert!(git.remote_web_url("origin").is_err());
}
