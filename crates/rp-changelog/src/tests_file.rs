use crate::{extract, prepend};
use semver::Version;

#[test]
fn prepend_creates_header_for_empty_file() {
    let out = prepend("", "## 1.0.0 (2026-01-01)\n");
    assert_eq!(out, "# Changelog\n\n## 1.0.0 (2026-01-01)\n");
}

#[test]
fn prepend_inserts_above_existing_entries_and_keeps_header() {
    let existing = "# Changelog\n\n## 1.0.0 (2026-01-01)\n\n### Features\n\n* a\n";
    let out = prepend(existing, "## 1.1.0 (2026-02-01)\n");
    assert_eq!(
        out,
        "# Changelog\n\n## 1.1.0 (2026-02-01)\n\n## 1.0.0 (2026-01-01)\n\n### Features\n\n* a\n"
    );
}

#[test]
fn prepend_header_only_file() {
    let out = prepend("# Changelog\n", "## 1.1.0 (2026-02-01)\n");
    assert_eq!(out, "# Changelog\n\n## 1.1.0 (2026-02-01)\n");
}

#[test]
fn prepend_adds_header_when_missing() {
    let out = prepend("## 1.0.0 (2026-01-01)\n", "## 1.1.0 (2026-02-01)\n");
    assert_eq!(
        out,
        "# Changelog\n\n## 1.1.0 (2026-02-01)\n\n## 1.0.0 (2026-01-01)\n"
    );
}

#[test]
fn extract_bracketed_and_bare_versions() {
    let log = "# Changelog\n\n## [1.2.0](x) (2026-11-17)\n\n### Features\n\n* new\n\n## 1.1.0 (2026-10-01)\n\n* old\n";
    let v12 = Version::parse("1.2.0").unwrap();
    let v11 = Version::parse("1.1.0").unwrap();
    assert_eq!(extract(log, &v12).unwrap(), "### Features\n\n* new");
    assert_eq!(extract(log, &v11).unwrap(), "* old");
}

#[test]
fn extract_missing_version_is_none() {
    let log = "## [1.0.0](x) (2026-01-01)\n\n* a\n";
    assert!(extract(log, &Version::parse("2.0.0").unwrap()).is_none());
}

#[test]
fn extract_last_entry_runs_to_end_of_file() {
    let log = "## 1.0.0 (2026-01-01)\n\n* only\n";
    assert_eq!(
        extract(log, &Version::parse("1.0.0").unwrap()).unwrap(),
        "* only"
    );
}
