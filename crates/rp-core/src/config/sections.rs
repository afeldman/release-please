//! Default changelog sections (release-please order and visibility).

use super::ChangelogSection;

/// Default sections in release-please order.
pub fn default_sections() -> Vec<ChangelogSection> {
    const TABLE: &[(&str, &str, bool)] = &[
        ("feat", "Features", false),
        ("fix", "Bug Fixes", false),
        ("perf", "Performance Improvements", false),
        ("revert", "Reverts", false),
        ("docs", "Documentation", true),
        ("style", "Styles", true),
        ("chore", "Miscellaneous Chores", true),
        ("refactor", "Code Refactoring", true),
        ("test", "Tests", true),
        ("build", "Build System", true),
        ("ci", "Continuous Integration", true),
    ];
    TABLE
        .iter()
        .map(|(kind, section, hidden)| ChangelogSection {
            kind: (*kind).to_string(),
            section: (*section).to_string(),
            hidden: *hidden,
        })
        .collect()
}
