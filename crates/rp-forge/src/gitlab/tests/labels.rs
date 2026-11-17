use super::{PROJECT_PATH, client};
use crate::Forge;
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn add_labels_sends_comma_separated_add_labels() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path(format!("{PROJECT_PATH}/merge_requests/7"))
            .header("PRIVATE-TOKEN", "test-token")
            .json_body(json!({"add_labels": "autorelease: pending,release"}));
        then.status(200).json_body(json!({}));
    });
    client(&server)
        .add_labels(7, &["autorelease: pending", "release"])
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn remove_label_sends_remove_labels() {
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path(format!("{PROJECT_PATH}/merge_requests/7"))
            .json_body(json!({"remove_labels": "autorelease: pending"}));
        then.status(200).json_body(json!({}));
    });
    client(&server)
        .remove_label(7, "autorelease: pending")
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn add_labels_empty_makes_no_request() {
    // Nothing listens on port 1: any request would fail the test.
    let cfg =
        crate::test_util::config(crate::ForgeKind::Gitlab, "http://127.0.0.1:1", "afe/crater");
    crate::gitlab::Gitlab::new(cfg).add_labels(7, &[]).unwrap();
}
