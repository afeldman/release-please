use super::*;

const SHA: &str = "abc123";

fn one(message: &str) -> ParsedCommit {
    parse(SHA, message).expect("conventional")
}

#[test]
fn parses_type_scope_and_description() {
    let c = one("feat(api): add login");
    assert_eq!(c.kind, "feat");
    assert_eq!(c.scope.as_deref(), Some("api"));
    assert_eq!(c.description, "add login");
    assert_eq!(c.sha, SHA);
    assert!(!c.breaking);
    assert_eq!(c.breaking_description, None);
}

#[test]
fn kind_is_lowercased() {
    assert_eq!(one("FIX: patch the thing").kind, "fix");
}

#[test]
fn scope_is_none_when_absent() {
    assert_eq!(one("fix: x").scope, None);
}

#[test]
fn bang_marks_breaking_and_uses_description() {
    let c = one("feat!: drop v1 endpoints");
    assert!(c.breaking);
    assert_eq!(c.breaking_description.as_deref(), Some("drop v1 endpoints"));
}

#[test]
fn breaking_change_footer_is_breaking() {
    let c = one("feat: new api\n\nBREAKING CHANGE: config moved");
    assert!(c.breaking);
    assert_eq!(c.breaking_description.as_deref(), Some("config moved"));
}

#[test]
fn breaking_hyphen_footer_is_breaking() {
    let c = one("fix: x\n\nBREAKING-CHANGE: renamed flag");
    assert!(c.breaking);
    assert_eq!(c.breaking_description.as_deref(), Some("renamed flag"));
}

#[test]
fn non_conventional_message_is_none() {
    assert!(parse(SHA, "Merge branch 'main'").is_none());
    assert!(parse(SHA, "").is_none());
}

#[test]
fn body_is_kept() {
    let c = one("fix: a\n\nlonger explanation");
    assert_eq!(c.body.as_deref(), Some("longer explanation"));
}

#[test]
fn body_is_none_without_body() {
    assert_eq!(one("fix: a").body, None);
}

#[test]
fn footers_are_listed_in_order() {
    let c = one("fix: a\n\nReviewed-by: Z\nRelease-As: 2.0.0");
    assert_eq!(
        c.footers,
        vec![
            ("Reviewed-by".to_string(), "Z".to_string()),
            ("Release-As".to_string(), "2.0.0".to_string()),
        ]
    );
}

#[test]
fn trailing_newline_is_accepted() {
    let c = parse(SHA, "feat: a\n").expect("trailing newline ok");
    assert_eq!(c.description, "a");
}

#[test]
fn release_as_footer_is_read() {
    let c = one("fix: a\n\nRelease-As: 2.0.0");
    assert_eq!(release_as(&[c]), Some(semver::Version::new(2, 0, 0)));
}

#[test]
fn release_as_token_is_case_insensitive() {
    let c = one("fix: a\n\nrelease-as: 3.1.0");
    assert_eq!(release_as(&[c]), Some(semver::Version::new(3, 1, 0)));
}

#[test]
fn release_as_newest_commit_wins() {
    let older = one("fix: a\n\nRelease-As: 1.0.0");
    let newer = one("fix: b\n\nRelease-As: 4.0.0");
    assert_eq!(
        release_as(&[newer, older]),
        Some(semver::Version::new(4, 0, 0))
    );
}

#[test]
fn release_as_skips_invalid_value() {
    let good = one("fix: a\n\nRelease-As: 1.2.0");
    let bad = one("fix: b\n\nRelease-As: not-a-version");
    assert_eq!(
        release_as(&[good, bad]),
        Some(semver::Version::new(1, 2, 0))
    );
}

#[test]
fn release_as_accepts_leading_v() {
    let c = one("fix: a\n\nRelease-As: v2.0.0");
    assert_eq!(release_as(&[c]), Some(semver::Version::new(2, 0, 0)));
}

#[test]
fn release_as_none_without_footer() {
    assert_eq!(release_as(&[one("feat: a")]), None);
    assert_eq!(release_as(&[]), None);
}

#[test]
fn breaking_footer_is_not_release_as() {
    let c = one("feat: a\n\nBREAKING CHANGE: x");
    assert_eq!(release_as(&[c]), None);
}
