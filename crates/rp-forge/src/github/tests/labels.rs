use super::{REPO_PATH, client};
use crate::Forge;
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn add_labels_posts_names_in_one_call() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/issues/21/labels"))
            .header("Authorization", "Bearer test-token")
            .json_body(json!({"labels": ["autorelease: pending", "release"]}));
        then.status(200)
            .json_body(json!([{"name": "autorelease: pending"}]));
    });
    client(&server)
        .add_labels(21, &["autorelease: pending", "release"])
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn add_labels_empty_makes_no_request() {
    // Nothing listens on port 1: any request would fail the test.
    let cfg =
        crate::test_util::config(crate::ForgeKind::Github, "http://127.0.0.1:1", "afe/crater");
    crate::github::Github::new(cfg).add_labels(21, &[]).unwrap();
}

#[test]
fn remove_label_url_encodes_name() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!(
                "{REPO_PATH}/issues/21/labels/autorelease%3A%20pending"
            ))
            .header("Authorization", "Bearer test-token");
        then.status(200).json_body(json!([]));
    });
    client(&server)
        .remove_label(21, "autorelease: pending")
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn remove_label_not_attached_is_ok() {
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("{REPO_PATH}/issues/21/labels/release"));
        then.status(404)
            .json_body(json!({"message": "Label does not exist"}));
    });
    client(&server).remove_label(21, "release").unwrap();
}
