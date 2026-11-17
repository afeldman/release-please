//! Chart.yaml, version.txt and generic marker updaters.

use rp_updaters::Updater;
use rp_updaters::chart_yaml::ChartYaml;
use rp_updaters::generic::GenericMarkers;
use rp_updaters::version_txt::VersionTxt;
use semver::Version;

fn v(s: &str) -> Version {
    Version::parse(s).expect("valid test version")
}

const CHART: &str = "\
apiVersion: v2
name: demo
description: A chart   # comment
type: application
version: 0.1.0  # chart version
appVersion: \"1.16.0\"
dependencies:
  - name: redis
    version: 17.0.0
    repository: https://charts.example
";

#[test]
fn chart_updates_version_and_app_version_keeping_quotes_and_comments() {
    let expected = "\
apiVersion: v2
name: demo
description: A chart   # comment
type: application
version: 0.2.0  # chart version
appVersion: \"0.2.0\"
dependencies:
  - name: redis
    version: 17.0.0
    repository: https://charts.example
";
    assert_eq!(
        ChartYaml.update(CHART, &v("0.2.0")).expect("update"),
        expected
    );
}

#[test]
fn chart_without_app_version_only_changes_version() {
    let input = "name: demo\nversion: '0.1.0'\n";
    let expected = "name: demo\nversion: '0.2.0'\n";
    assert_eq!(
        ChartYaml.update(input, &v("0.2.0")).expect("update"),
        expected
    );
}

#[test]
fn chart_nested_version_is_not_used_as_top_level() {
    let input = "dependencies:\n  - name: redis\n    version: 17.0.0\n";
    assert!(ChartYaml.update(input, &v("1.0.0")).is_err());
}

#[test]
fn chart_with_crlf_line_endings_is_preserved() {
    let input = "name: demo\r\nversion: 0.1.0\r\nappVersion: 1.0\r\n";
    let expected = "name: demo\r\nversion: 0.2.0\r\nappVersion: 0.2.0\r\n";
    assert_eq!(
        ChartYaml.update(input, &v("0.2.0")).expect("update"),
        expected
    );
}

#[test]
fn version_txt_is_replaced_with_newline() {
    assert_eq!(
        VersionTxt.update("0.1.0\n", &v("0.2.0")).expect("update"),
        "0.2.0\n"
    );
}

#[test]
fn version_txt_adds_missing_newline() {
    assert_eq!(
        VersionTxt
            .update("0.1.0", &v("1.0.0-rc.1"))
            .expect("update"),
        "1.0.0-rc.1\n"
    );
}

#[test]
fn generic_version_marker_changes_only_marked_line() {
    let input = "// comment\npub const VERSION: &str = \"1.2.3\"; // x-release-please-version\nother = \"1.2.3\";\n";
    let expected = "// comment\npub const VERSION: &str = \"2.0.0\"; // x-release-please-version\nother = \"1.2.3\";\n";
    assert_eq!(
        GenericMarkers.update(input, &v("2.0.0")).expect("update"),
        expected
    );
}

#[test]
fn generic_start_end_block_replaces_every_line_inside() {
    let input = "\
<!-- x-release-please-start-version -->
Current: 1.2.3
Also: 1.2.3-beta.1
<!-- x-release-please-end -->
Old: 1.2.3
";
    let expected = "\
<!-- x-release-please-start-version -->
Current: 2.0.0
Also: 2.0.0
<!-- x-release-please-end -->
Old: 1.2.3
";
    assert_eq!(
        GenericMarkers.update(input, &v("2.0.0")).expect("update"),
        expected
    );
}

#[test]
fn generic_keeps_prerelease_and_build_suffix_out_of_neighbouring_text() {
    let input = "image: app:1.2.3-rc.1 # x-release-please-version\n";
    let expected = "image: app:1.3.0 # x-release-please-version\n";
    assert_eq!(
        GenericMarkers.update(input, &v("1.3.0")).expect("update"),
        expected
    );
}

#[test]
fn generic_major_minor_patch_markers_replace_one_component_each() {
    let input = "\
major: 1.2.3 # x-release-please-major
minor: 1.2.3 # x-release-please-minor
patch: 1.2.3 # x-release-please-patch
";
    let expected = "\
major: 3.2.3 # x-release-please-major
minor: 1.4.3 # x-release-please-minor
patch: 1.2.5 # x-release-please-patch
";
    assert_eq!(
        GenericMarkers.update(input, &v("3.4.5")).expect("update"),
        expected
    );
}

#[test]
fn generic_marker_without_semver_is_an_error() {
    let input = "version: none # x-release-please-version\n";
    assert!(GenericMarkers.update(input, &v("1.0.0")).is_err());
}

#[test]
fn generic_without_markers_is_unchanged() {
    let input = "version = \"1.2.3\"\n";
    assert_eq!(
        GenericMarkers.update(input, &v("9.9.9")).expect("update"),
        input
    );
}
