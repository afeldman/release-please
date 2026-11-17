use super::{REPO_PATH, client};
use crate::{Forge, NewRelease};
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn create_release_posts_full_payload() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/releases"))
            .header("Authorization", "token test-token")
            .json_body(json!({
                "tag_name": "v1.2.0",
                "target_commitish": "deadbeef",
                "name": "v1.2.0",
                "body": "## Features",
                "draft": false,
                "prerelease": true
            }));
        then.status(201)
            .json_body(json!({"id": 1, "tag_name": "v1.2.0"}));
    });
    let release = NewRelease {
        tag: "v1.2.0",
        target_sha: "deadbeef",
        name: "v1.2.0",
        body: "## Features",
        draft: false,
        prerelease: true,
    };
    client(&server).create_release(&release).unwrap();
    mock.assert_hits(1);
}

#[test]
fn create_release_error_names_the_action() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path(format!("{REPO_PATH}/releases"));
        then.status(409).body("tag exists");
    });
    let release = NewRelease {
        tag: "v1",
        target_sha: "x",
        name: "v1",
        body: "",
        draft: false,
        prerelease: false,
    };
    let err = client(&server)
        .create_release(&release)
        .unwrap_err()
        .to_string();
    assert!(err.contains("create release"), "{err}");
    assert!(err.contains("HTTP 409"), "{err}");
}

#[test]
fn release_exists_true_on_200() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/releases/tags/v1.2.0"))
            .header("Authorization", "token test-token");
        then.status(200).json_body(json!({"tag_name": "v1.2.0"}));
    });
    assert!(client(&server).release_exists("v1.2.0").unwrap());
}

#[test]
fn release_exists_false_on_404() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/releases/tags/v9.9.9"));
        then.status(404);
    });
    assert!(!client(&server).release_exists("v9.9.9").unwrap());
}

#[test]
fn release_exists_encodes_slashed_tag() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/releases/tags/pkg%2Fv1.0.0"));
        then.status(404);
    });
    assert!(!client(&server).release_exists("pkg/v1.0.0").unwrap());
    mock.assert_hits(1);
}
