use super::*;

fn commit(kind: &str, breaking: bool) -> ParsedCommit {
    ParsedCommit {
        sha: "sha".into(),
        kind: kind.into(),
        scope: None,
        description: "d".into(),
        body: None,
        breaking,
        breaking_description: None,
        footers: vec![],
    }
}

fn with_footer(mut c: ParsedCommit, token: &str, value: &str) -> ParsedCommit {
    c.footers.push((token.into(), value.into()));
    c
}

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid version")
}

fn cfg() -> PackageConfig {
    PackageConfig::default()
}

fn rc_cfg() -> PackageConfig {
    PackageConfig {
        prerelease_type: Some("rc".into()),
        ..cfg()
    }
}

fn pre_major_cfg(minor: bool, patch: bool) -> PackageConfig {
    PackageConfig {
        bump_minor_pre_major: minor,
        bump_patch_for_minor_pre_major: patch,
        ..cfg()
    }
}

fn next(prev: &str, kinds: &[(&str, bool)], c: &PackageConfig) -> Option<Version> {
    let commits: Vec<_> = kinds.iter().map(|(k, b)| commit(k, *b)).collect();
    next_version(Some(&v(prev)), &commits, c)
}

// --- bump_for ---

#[test]
fn bump_for_empty_is_none() {
    assert_eq!(bump_for(&[]), Bump::None);
}

#[test]
fn bump_for_breaking_is_major_even_for_chore() {
    assert_eq!(bump_for(&[commit("chore", true)]), Bump::Major);
}

#[test]
fn bump_for_feat_is_minor() {
    assert_eq!(bump_for(&[commit("feat", false)]), Bump::Minor);
}

#[test]
fn bump_for_fix_perf_revert_are_patch() {
    for kind in ["fix", "perf", "revert"] {
        assert_eq!(bump_for(&[commit(kind, false)]), Bump::Patch, "{kind}");
    }
}

#[test]
fn bump_for_other_types_are_none() {
    for kind in ["docs", "chore", "refactor", "test", "ci", "build", "style"] {
        assert_eq!(bump_for(&[commit(kind, false)]), Bump::None, "{kind}");
    }
}

#[test]
fn bump_for_highest_wins() {
    let commits = [
        commit("fix", false),
        commit("feat", false),
        commit("docs", false),
    ];
    assert_eq!(bump_for(&commits), Bump::Minor);
}

#[test]
fn bump_for_kind_is_case_insensitive() {
    assert_eq!(bump_for(&[commit("FEAT", false)]), Bump::Minor);
}

// --- stable releases ---

#[test]
fn stable_patch_minor_major() {
    assert_eq!(next("1.2.3", &[("fix", false)], &cfg()), Some(v("1.2.4")));
    assert_eq!(next("1.2.3", &[("feat", false)], &cfg()), Some(v("1.3.0")));
    assert_eq!(next("1.2.3", &[("fix", true)], &cfg()), Some(v("2.0.0")));
}

#[test]
fn no_bump_and_no_override_is_none() {
    assert_eq!(next("1.2.3", &[("chore", false)], &cfg()), None);
    assert_eq!(next_version(None, &[commit("docs", false)], &cfg()), None);
}

#[test]
fn config_release_as_overrides_commits() {
    let c = PackageConfig {
        release_as: Some("5.0.0".into()),
        ..cfg()
    };
    assert_eq!(next("1.2.3", &[("fix", false)], &c), Some(v("5.0.0")));
}

#[test]
fn config_release_as_applies_without_bump() {
    let c = PackageConfig {
        release_as: Some("0.9.0".into()),
        ..cfg()
    };
    assert_eq!(next("1.2.3", &[("chore", false)], &c), Some(v("0.9.0")));
}

#[test]
fn release_as_footer_overrides_commits() {
    let commits = [with_footer(commit("fix", false), "Release-As", "3.0.0")];
    assert_eq!(
        next_version(Some(&v("1.2.3")), &commits, &cfg()),
        Some(v("3.0.0"))
    );
}

#[test]
fn config_release_as_wins_over_footer() {
    let c = PackageConfig {
        release_as: Some("7.0.0".into()),
        ..cfg()
    };
    let commits = [with_footer(commit("fix", false), "Release-As", "3.0.0")];
    assert_eq!(
        next_version(Some(&v("1.2.3")), &commits, &c),
        Some(v("7.0.0"))
    );
}

#[test]
fn invalid_config_release_as_falls_back_to_bump() {
    let c = PackageConfig {
        release_as: Some("soon".into()),
        ..cfg()
    };
    assert_eq!(next("1.2.3", &[("fix", false)], &c), Some(v("1.2.4")));
}

// --- initial version ---

#[test]
fn initial_version_defaults_to_1_0_0() {
    assert_eq!(
        next_version(None, &[commit("feat", false)], &cfg()),
        Some(v("1.0.0"))
    );
}

#[test]
fn initial_version_is_0_1_0_when_pre_major_minor_bump() {
    let c = pre_major_cfg(true, false);
    assert_eq!(
        next_version(None, &[commit("feat", false)], &c),
        Some(v("0.1.0"))
    );
}

#[test]
fn explicit_initial_version_is_used() {
    let c = PackageConfig {
        initial_version: Some("2.5.0".into()),
        ..cfg()
    };
    assert_eq!(
        next_version(None, &[commit("fix", false)], &c),
        Some(v("2.5.0"))
    );
}

#[test]
fn initial_version_not_bumped_from_commits() {
    assert_eq!(
        next_version(None, &[commit("feat", true)], &cfg()),
        Some(v("1.0.0"))
    );
}

// --- pre-major rules ---

#[test]
fn pre_major_breaking_bumps_minor_when_flag_set() {
    let c = pre_major_cfg(true, false);
    assert_eq!(next("0.4.2", &[("feat", true)], &c), Some(v("0.5.0")));
}

#[test]
fn pre_major_breaking_bumps_major_without_flag() {
    assert_eq!(next("0.4.2", &[("feat", true)], &cfg()), Some(v("1.0.0")));
}

#[test]
fn pre_major_feat_bumps_patch_when_flag_set() {
    let c = pre_major_cfg(false, true);
    assert_eq!(next("0.4.2", &[("feat", false)], &c), Some(v("0.4.3")));
}

#[test]
fn pre_major_feat_bumps_minor_without_flag() {
    assert_eq!(next("0.4.2", &[("feat", false)], &cfg()), Some(v("0.5.0")));
}

#[test]
fn pre_major_fix_is_unaffected_by_flags() {
    let c = pre_major_cfg(true, true);
    assert_eq!(next("0.4.2", &[("fix", false)], &c), Some(v("0.4.3")));
}

#[test]
fn pre_major_flags_do_not_apply_from_1_x() {
    let c = pre_major_cfg(true, true);
    assert_eq!(next("1.2.3", &[("feat", true)], &c), Some(v("2.0.0")));
    assert_eq!(next("1.2.3", &[("feat", false)], &c), Some(v("1.3.0")));
}

// --- prerelease (-rc.N) ---

#[test]
fn rc_from_stable_bumps_core_and_starts_at_zero() {
    assert_eq!(
        next("1.2.3", &[("feat", false)], &rc_cfg()),
        Some(v("1.3.0-rc.0"))
    );
}

#[test]
fn rc_initial_version_gets_suffix() {
    assert_eq!(
        next_version(None, &[commit("feat", false)], &rc_cfg()),
        Some(v("1.0.0-rc.0"))
    );
}

#[test]
fn rc_continuation_increments_counter() {
    assert_eq!(
        next("1.3.0-rc.0", &[("feat", false)], &rc_cfg()),
        Some(v("1.3.0-rc.1"))
    );
}

#[test]
fn rc_fix_on_same_core_keeps_core() {
    assert_eq!(
        next("1.3.0-rc.1", &[("fix", false)], &rc_cfg()),
        Some(v("1.3.0-rc.2"))
    );
}

#[test]
fn rc_feat_after_patch_rc_bumps_minor() {
    assert_eq!(
        next("1.3.1-rc.0", &[("feat", false)], &rc_cfg()),
        Some(v("1.4.0-rc.0"))
    );
}

#[test]
fn rc_breaking_exceeds_minor_core() {
    assert_eq!(
        next("1.3.0-rc.1", &[("feat", true)], &rc_cfg()),
        Some(v("2.0.0-rc.0"))
    );
}

#[test]
fn rc_breaking_on_major_core_keeps_core() {
    assert_eq!(
        next("2.0.0-rc.1", &[("feat", true)], &rc_cfg()),
        Some(v("2.0.0-rc.2"))
    );
}

#[test]
fn rc_pre_major_feat_keeps_minor_core() {
    assert_eq!(
        next("0.5.0-rc.1", &[("feat", false)], &rc_cfg()),
        Some(v("0.5.0-rc.2"))
    );
}

#[test]
fn rc_pre_major_breaking_with_flag_keeps_minor_core() {
    let c = PackageConfig {
        bump_minor_pre_major: true,
        ..rc_cfg()
    };
    assert_eq!(
        next("0.5.0-rc.1", &[("feat", true)], &c),
        Some(v("0.5.0-rc.2"))
    );
}

#[test]
fn rc_graduates_to_stable_without_type() {
    assert_eq!(
        next("1.3.0-rc.1", &[("fix", false)], &cfg()),
        Some(v("1.3.0"))
    );
}

#[test]
fn rc_patch_core_graduates_on_fix_without_type() {
    assert_eq!(
        next("1.3.1-rc.0", &[("fix", false)], &cfg()),
        Some(v("1.3.1"))
    );
}

#[test]
fn rc_feat_on_minor_core_graduates_without_type() {
    assert_eq!(
        next("1.3.0-rc.1", &[("feat", false)], &cfg()),
        Some(v("1.3.0"))
    );
}

#[test]
fn rc_other_prerelease_type_restarts_counter() {
    assert_eq!(
        next("1.3.0-beta.2", &[("feat", false)], &rc_cfg()),
        Some(v("1.3.0-rc.0"))
    );
}

#[test]
fn rc_non_numeric_prerelease_restarts_counter() {
    assert_eq!(
        next("1.3.0-rc", &[("feat", false)], &rc_cfg()),
        Some(v("1.3.0-rc.0"))
    );
}

#[test]
fn rc_no_bump_is_none() {
    assert_eq!(next("1.3.0-rc.1", &[("chore", false)], &rc_cfg()), None);
}

#[test]
fn rc_release_as_is_exact_without_suffix() {
    let c = PackageConfig {
        release_as: Some("2.0.0".into()),
        ..rc_cfg()
    };
    assert_eq!(next("1.3.0-rc.1", &[("fix", false)], &c), Some(v("2.0.0")));
}

#[test]
fn empty_prerelease_type_is_unset() {
    let c = PackageConfig {
        prerelease_type: Some(String::new()),
        ..cfg()
    };
    assert_eq!(next("1.2.3", &[("fix", false)], &c), Some(v("1.2.4")));
}

#[test]
fn invalid_prerelease_type_yields_none() {
    let c = PackageConfig {
        prerelease_type: Some("rc 1".into()),
        ..cfg()
    };
    assert_eq!(next("1.2.3", &[("fix", false)], &c), None);
}
