use crate::{EntryInput, render_entry};
use rp_core::{ChangelogSection, ParsedCommit};
use semver::Version;

const REPO: &str = "https://git.example/org/repo";

fn commit(kind: &str, scope: Option<&str>, desc: &str, sha: &str) -> ParsedCommit {
    ParsedCommit {
        sha: sha.to_string(),
        kind: kind.to_string(),
        scope: scope.map(str::to_string),
        description: desc.to_string(),
        body: None,
        breaking: false,
        breaking_description: None,
        footers: Vec::new(),
    }
}

fn section(kind: &str, title: &str, hidden: bool) -> ChangelogSection {
    ChangelogSection {
        kind: kind.to_string(),
        section: title.to_string(),
        hidden,
    }
}

pub(super) fn sections() -> Vec<ChangelogSection> {
    vec![
        section("feat", "Features", false),
        section("fix", "Bug Fixes", false),
        section("chore", "Miscellaneous", true),
    ]
}

pub(super) fn commits() -> Vec<ParsedCommit> {
    let mut breaking = commit("feat", Some("api"), "add x", "abc1234def0");
    breaking.breaking = true;
    breaking.breaking_description = Some("drop old API".to_string());
    vec![
        commit("chore", None, "tidy", "2222222bbbb"),
        commit("fix", None, "fix y", "1111111aaaa"),
        breaking,
    ]
}

pub(super) fn input<'a>(
    version: &'a Version,
    commits: &'a [ParsedCommit],
    sections: &'a [ChangelogSection],
) -> EntryInput<'a> {
    EntryInput {
        version,
        previous_tag: Some("v1.1.0"),
        tag: "v1.2.0",
        date: "2026-11-17",
        commits,
        sections,
        repo_url: Some(REPO),
        template: None,
    }
}

#[test]
fn default_entry_matches_release_please_layout() {
    let v = Version::parse("1.2.0").unwrap();
    let (c, s) = (commits(), sections());
    let out = render_entry(&input(&v, &c, &s)).unwrap();
    let expected = "\
## [1.2.0](https://git.example/org/repo/compare/v1.1.0...v1.2.0) (2026-11-17)

### ⚠ BREAKING CHANGES

* **api:** drop old API

### Features

* **api:** add x ([abc1234](https://git.example/org/repo/commit/abc1234def0))

### Bug Fixes

* fix y ([1111111](https://git.example/org/repo/commit/1111111aaaa))
";
    assert_eq!(out, expected);
}

#[test]
fn no_repo_url_drops_heading_and_commit_links() {
    let v = Version::parse("1.2.0").unwrap();
    let (c, s) = (commits(), sections());
    let mut inp = input(&v, &c, &s);
    inp.repo_url = None;
    let out = render_entry(&inp).unwrap();
    assert!(out.starts_with("## 1.2.0 (2026-11-17)\n"));
    assert!(out.contains("* fix y (1111111)\n"));
    assert!(!out.contains("]("));
}

#[test]
fn no_previous_tag_drops_compare_link_only() {
    let v = Version::parse("1.2.0").unwrap();
    let (c, s) = (commits(), sections());
    let mut inp = input(&v, &c, &s);
    inp.previous_tag = None;
    let out = render_entry(&inp).unwrap();
    assert!(out.starts_with("## 1.2.0 (2026-11-17)\n"));
    assert!(out.contains("([1111111](https://git.example/org/repo/commit/1111111aaaa))"));
}

#[test]
fn hidden_and_unknown_types_are_skipped_and_empty_sections_dropped() {
    let v = Version::parse("1.0.0").unwrap();
    let c = vec![
        commit("chore", None, "tidy", "2222222bbbb"),
        commit("docs", None, "x", "3333333cccc"),
    ];
    let s = sections();
    let out = render_entry(&input(&v, &c, &s)).unwrap();
    assert!(!out.contains("Miscellaneous"));
    assert!(!out.contains("tidy"));
    assert!(!out.contains("###"));
}

#[test]
fn sections_follow_config_order_not_commit_order() {
    let v = Version::parse("1.0.0").unwrap();
    let c = vec![
        commit("fix", None, "first fix", "aaaaaaa1111"),
        commit("feat", None, "a feature", "bbbbbbb2222"),
    ];
    let s = sections();
    let out = render_entry(&input(&v, &c, &s)).unwrap();
    let feat = out.find("### Features").unwrap();
    let fix = out.find("### Bug Fixes").unwrap();
    assert!(feat < fix);
}
