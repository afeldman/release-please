use super::*;

fn pkg(path: &str) -> PackageConfig {
    PackageConfig {
        path: path.to_string(),
        ..Default::default()
    }
}

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid version")
}

#[test]
fn root_tag_has_v_prefix_and_no_component() {
    assert_eq!(tag_name(&pkg("."), &v("1.2.3")), "v1.2.3");
}

#[test]
fn v_prefix_can_be_disabled() {
    let cfg = PackageConfig {
        include_v_in_tag: Some(false),
        ..pkg(".")
    };
    assert_eq!(tag_name(&cfg, &v("1.2.3")), "1.2.3");
}

#[test]
fn component_defaults_to_last_path_segment() {
    assert_eq!(tag_name(&pkg("crates/titan"), &v("0.2.0")), "titan-v0.2.0");
}

#[test]
fn trailing_slash_in_path_is_ignored() {
    assert_eq!(component_name(&pkg("pkgs/foo/")).as_deref(), Some("foo"));
}

#[test]
fn package_name_beats_path_segment() {
    let cfg = PackageConfig {
        package_name: Some("titan-core".into()),
        ..pkg("crates/x")
    };
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "titan-core-v1.0.0");
}

#[test]
fn explicit_component_beats_package_name() {
    let cfg = PackageConfig {
        component: Some("api".into()),
        package_name: Some("ignored".into()),
        ..pkg("crates/x")
    };
    assert_eq!(component_name(&cfg).as_deref(), Some("api"));
}

#[test]
fn root_has_no_default_component() {
    assert_eq!(component_name(&pkg(".")), None);
    assert_eq!(component_name(&pkg("")), None);
}

#[test]
fn root_ignores_package_name_for_component() {
    let cfg = PackageConfig {
        package_name: Some("mylib".into()),
        ..pkg(".")
    };
    assert_eq!(component_name(&cfg), None);
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "v1.0.0");
}

#[test]
fn explicit_component_on_root_is_not_in_tag_by_default() {
    let cfg = PackageConfig {
        component: Some("core".into()),
        ..pkg(".")
    };
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "v1.0.0");
}

#[test]
fn explicit_component_on_root_included_when_flag_set() {
    let cfg = PackageConfig {
        component: Some("core".into()),
        include_component_in_tag: Some(true),
        ..pkg(".")
    };
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "core-v1.0.0");
}

#[test]
fn include_component_false_drops_component() {
    let cfg = PackageConfig {
        include_component_in_tag: Some(false),
        ..pkg("a/b")
    };
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "v1.0.0");
}

#[test]
fn custom_separator_is_used() {
    let cfg = PackageConfig {
        tag_separator: Some("_".into()),
        ..pkg("crates/titan")
    };
    assert_eq!(tag_name(&cfg, &v("1.0.0")), "titan_v1.0.0");
}

#[test]
fn prerelease_is_kept_in_tag() {
    assert_eq!(
        tag_name(&pkg("crates/titan"), &v("1.0.0-rc.1")),
        "titan-v1.0.0-rc.1"
    );
}

#[test]
fn parse_roundtrips_generated_tag() {
    let cfg = pkg("crates/titan");
    let tag = tag_name(&cfg, &v("0.4.7"));
    assert_eq!(parse_tag(&cfg, &tag), Some(v("0.4.7")));
}

#[test]
fn parse_accepts_missing_v() {
    assert_eq!(parse_tag(&pkg("."), "1.2.3"), Some(v("1.2.3")));
    assert_eq!(
        parse_tag(&pkg("crates/titan"), "titan-1.2.3"),
        Some(v("1.2.3"))
    );
}

#[test]
fn parse_root_plain_tag() {
    assert_eq!(parse_tag(&pkg("."), "v1.2.3"), Some(v("1.2.3")));
}

#[test]
fn parse_prerelease_tag() {
    assert_eq!(
        parse_tag(&pkg("crates/titan"), "titan-v1.0.0-rc.1"),
        Some(v("1.0.0-rc.1"))
    );
}

#[test]
fn parse_rejects_other_component() {
    assert_eq!(parse_tag(&pkg("crates/titan"), "other-v1.0.0"), None);
}

#[test]
fn parse_rejects_unprefixed_tag_for_component_package() {
    assert_eq!(parse_tag(&pkg("crates/titan"), "v1.0.0"), None);
}

#[test]
fn parse_root_rejects_component_tag() {
    assert_eq!(parse_tag(&pkg("."), "titan-v1.0.0"), None);
}

#[test]
fn parse_rejects_double_v() {
    assert_eq!(parse_tag(&pkg("."), "vv1.0.0"), None);
}

#[test]
fn parse_rejects_non_version() {
    assert_eq!(parse_tag(&pkg("crates/titan"), "titan-vfoo"), None);
    assert_eq!(parse_tag(&pkg("."), "latest"), None);
}

#[test]
fn parse_respects_custom_separator() {
    let cfg = PackageConfig {
        tag_separator: Some("/".into()),
        ..pkg("crates/titan")
    };
    assert_eq!(parse_tag(&cfg, "titan/v1.0.0"), Some(v("1.0.0")));
    assert_eq!(parse_tag(&cfg, "titan-v1.0.0"), None);
}
