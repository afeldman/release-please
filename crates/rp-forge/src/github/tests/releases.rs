use super::{REPO_PATH, client};
use crate::{Forge, NewRelease};
use httpmock::prelude::*;
use serde_json::json;

fn release<'a>(tag: &'a str) -> NewRelease<'a> {
    NewRelease {
        tag,
        target_sha: "deadbeef",
        name: tag,
        body: "## Bug Fixes",
        draft: false,
        prerelease: false,
    }
}

#[test]
fn create_release_posts_payload() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/releases"))
            .json_body(json!({
                "tag_name": "v1.2.0",
                "target_commitish": "deadbeef",
                "name": "v1.2.0",
                "body": "## Bug Fixes",
                "draft": false,
                "prerelease": false
            }));
        then.status(201).json_body(json!({"id": 1}));
    });
    client(&server).create_release(&release("v1.2.0")).unwrap();
    mock.assert_hits(1);
}

#[test]
fn release_exists_reports_200_and_404() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/releases/tags/v1.2.0"))
            .header("Authorization", "Bearer test-token");
        then.status(200).json_body(json!({"tag_name": "v1.2.0"}));
    });
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/releases/tags/v9.9.9"));
        then.status(404);
    });
    let forge = client(&server);
    assert!(forge.release_exists("v1.2.0").unwrap());
    assert!(!forge.release_exists("v9.9.9").unwrap());
}
