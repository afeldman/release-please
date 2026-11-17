use super::{PROJECT_PATH, client};
use crate::{Forge, NewRelease};
use httpmock::prelude::*;
use serde_json::json;

fn release<'a>(tag: &'a str, draft: bool) -> NewRelease<'a> {
    NewRelease {
        tag,
        target_sha: "deadbeef",
        name: tag,
        body: "## Bug Fixes",
        draft,
        prerelease: true,
    }
}

#[test]
fn create_release_maps_target_to_ref_and_body_to_description() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{PROJECT_PATH}/releases"))
            .header("PRIVATE-TOKEN", "test-token")
            .json_body(json!({
                "tag_name": "v1.2.0",
                "ref": "deadbeef",
                "name": "v1.2.0",
                "description": "## Bug Fixes"
            }));
        then.status(201).json_body(json!({"tag_name": "v1.2.0"}));
    });
    client(&server)
        .create_release(&release("v1.2.0", false))
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn create_draft_release_is_rejected_without_request() {
    // Nothing listens on port 1: a request would fail with a different error.
    let cfg =
        crate::test_util::config(crate::ForgeKind::Gitlab, "http://127.0.0.1:1", "afe/crater");
    let err = crate::gitlab::Gitlab::new(cfg)
        .create_release(&release("v1.2.0", true))
        .unwrap_err()
        .to_string();
    assert!(err.contains("draft"), "{err}");
}

#[test]
fn release_exists_reports_200_and_404() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/releases/v1.2.0"));
        then.status(200).json_body(json!({"tag_name": "v1.2.0"}));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/releases/v9.9.9"));
        then.status(404);
    });
    let forge = client(&server);
    assert!(forge.release_exists("v1.2.0").unwrap());
    assert!(!forge.release_exists("v9.9.9").unwrap());
}

#[test]
fn release_exists_encodes_slashed_tag() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/releases/pkg%2Fv1.0.0"));
        then.status(404);
    });
    assert!(!client(&server).release_exists("pkg/v1.0.0").unwrap());
    mock.assert_hits(1);
}
