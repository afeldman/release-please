use crate::testutil::{commit_file, repo, sh};
use crate::{Git, RawCommit};

/// Layout (oldest first):
/// one: pkg/a.txt (tagged v1.0.0), two: other/b.txt,
/// three: pkg/skip/c.txt, four: pkg/d.txt
fn fixture() -> (tempfile::TempDir, Git) {
    let (tmp, git) = repo();
    let one = commit_file(&git, "pkg/a.txt", "a\n", "feat: one\n\nbody line");
    sh(&git.dir, &["tag", "v1.0.0", &one]);
    commit_file(&git, "other/b.txt", "b\n", "fix: two");
    commit_file(&git, "pkg/skip/c.txt", "c\n", "docs: three");
    commit_file(&git, "pkg/d.txt", "d\n", "feat: four");
    (tmp, git)
}

fn messages(commits: &[RawCommit]) -> Vec<&str> {
    commits.iter().map(|c| c.message.as_str()).collect()
}

#[test]
fn all_history_newest_first_with_full_body() {
    let (_tmp, git) = fixture();
    let commits = git.commits_since(None, ".", &[]).unwrap();
    assert_eq!(
        messages(&commits),
        vec![
            "feat: four",
            "docs: three",
            "fix: two",
            "feat: one\n\nbody line"
        ]
    );
    assert!(commits.iter().all(|c| c.sha.len() == 40));
}

#[test]
fn since_tag_excludes_tagged_commit() {
    let (_tmp, git) = fixture();
    let commits = git.commits_since(Some("v1.0.0"), ".", &[]).unwrap();
    assert_eq!(
        messages(&commits),
        vec!["feat: four", "docs: three", "fix: two"]
    );
}

#[test]
fn path_limits_history() {
    let (_tmp, git) = fixture();
    let commits = git.commits_since(None, "pkg", &[]).unwrap();
    assert_eq!(
        messages(&commits),
        vec!["feat: four", "docs: three", "feat: one\n\nbody line"]
    );
    let since = git.commits_since(Some("v1.0.0"), "pkg", &[]).unwrap();
    assert_eq!(messages(&since), vec!["feat: four", "docs: three"]);
}

#[test]
fn exclude_pathspec_removes_subtree() {
    let (_tmp, git) = fixture();
    let exclude = vec!["pkg/skip".to_string()];
    let in_pkg = git.commits_since(Some("v1.0.0"), "pkg", &exclude).unwrap();
    assert_eq!(messages(&in_pkg), vec!["feat: four"]);
    let all = git.commits_since(Some("v1.0.0"), ".", &exclude).unwrap();
    assert_eq!(messages(&all), vec!["feat: four", "fix: two"]);
}

#[test]
fn empty_range_and_invalid_revision() {
    let (_tmp, git) = fixture();
    assert!(
        git.commits_since(Some("HEAD"), ".", &[])
            .unwrap()
            .is_empty()
    );
    assert!(git.commits_since(Some("no-such-tag"), ".", &[]).is_err());
}
