use crate::testutil::{commit_file, repo, sh};

#[test]
fn tags_lists_lightweight_and_annotated() {
    let (_tmp, git) = repo();
    let sha = commit_file(&git, "a.txt", "a\n", "feat: first");
    sh(&git.dir, &["tag", "v1.0.0", &sha]);
    sh(
        &git.dir,
        &["tag", "-a", "v1.1.0", "-m", "release 1.1.0", &sha],
    );
    assert_eq!(git.tags().unwrap(), vec!["v1.0.0", "v1.1.0"]);
}

#[test]
fn tags_empty_repo_is_empty() {
    let (_tmp, git) = repo();
    assert!(git.tags().unwrap().is_empty());
}

#[test]
fn head_sha_matches_rev_parse() {
    let (_tmp, git) = repo();
    let sha = commit_file(&git, "a.txt", "a\n", "feat: first");
    assert_eq!(git.head_sha().unwrap(), sha);
    assert_eq!(sha.len(), 40);
}

#[test]
fn read_file_at_existing_missing_and_invalid_rev() {
    let (_tmp, git) = repo();
    commit_file(&git, "a.txt", "hello\n", "feat: first");
    assert_eq!(
        git.read_file_at("HEAD", "a.txt").unwrap().as_deref(),
        Some("hello\n")
    );
    assert_eq!(git.read_file_at("HEAD", "missing.txt").unwrap(), None);
    assert!(git.read_file_at("no-such-rev", "a.txt").is_err());
}

#[test]
fn checkout_new_branch_creates_and_resets() {
    let (_tmp, git) = repo();
    commit_file(&git, "a.txt", "a\n", "feat: first");
    git.checkout_new_branch("release/v1").unwrap();
    assert_eq!(
        sh(&git.dir, &["rev-parse", "--abbrev-ref", "HEAD"]),
        "release/v1"
    );
    // -B: running again must not fail when the branch exists.
    let second = commit_file(&git, "b.txt", "b\n", "feat: second");
    git.checkout_new_branch("release/v1").unwrap();
    assert_eq!(
        sh(&git.dir, &["rev-parse", "--abbrev-ref", "HEAD"]),
        "release/v1"
    );
    assert_eq!(git.head_sha().unwrap(), second);
}
