use super::{REPO_PATH, client};
use crate::Forge;
use httpmock::prelude::*;
use serde_json::json;

fn label_list(server: &httpmock::MockServer, labels: serde_json::Value) {
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/labels"))
            .query_param("page", "1");
        then.status(200).json_body(labels);
    });
}

#[test]
fn add_labels_creates_missing_and_attaches_ids() {
    let server = httpmock::MockServer::start();
    label_list(&server, json!([{"id": 3, "name": "autorelease: pending"}]));
    let create = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/labels"))
            .header("Authorization", "token test-token")
            .json_body(json!({"name": "autorelease: tagged", "color": "#ededed"}));
        then.status(201)
            .json_body(json!({"id": 8, "name": "autorelease: tagged"}));
    });
    let attach = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/issues/5/labels"))
            .json_body(json!({"labels": [3, 8]}));
        then.status(200).json_body(json!([]));
    });
    client(&server)
        .add_labels(5, &["autorelease: pending", "autorelease: tagged"])
        .unwrap();
    create.assert_hits(1);
    attach.assert_hits(1);
}

#[test]
fn add_labels_does_not_recreate_existing_labels() {
    let server = httpmock::MockServer::start();
    label_list(&server, json!([{"id": 3, "name": "autorelease: pending"}]));
    let create = server.mock(|when, then| {
        when.method(POST).path(format!("{REPO_PATH}/labels"));
        then.status(201).json_body(json!({"id": 99, "name": "x"}));
    });
    let attach = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/issues/5/labels"))
            .json_body(json!({"labels": [3]}));
        then.status(200).json_body(json!([]));
    });
    client(&server)
        .add_labels(5, &["autorelease: pending"])
        .unwrap();
    create.assert_hits(0);
    attach.assert_hits(1);
}

#[test]
fn add_labels_empty_makes_no_request() {
    // Nothing listens on port 1: any request would fail the test.
    let cfg = crate::test_util::config(crate::ForgeKind::Gitea, "http://127.0.0.1:1", "afe/crater");
    crate::gitea::Gitea::new(cfg).add_labels(5, &[]).unwrap();
}

#[test]
fn remove_label_deletes_by_id() {
    let server = httpmock::MockServer::start();
    label_list(&server, json!([{"id": 3, "name": "autorelease: pending"}]));
    let delete = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("{REPO_PATH}/issues/5/labels/3"))
            .header("Authorization", "token test-token");
        then.status(204);
    });
    client(&server)
        .remove_label(5, "autorelease: pending")
        .unwrap();
    delete.assert_hits(1);
}

#[test]
fn remove_label_unknown_to_repo_is_noop() {
    let server = httpmock::MockServer::start();
    label_list(&server, json!([]));
    let delete = server.mock(|when, then| {
        when.method(DELETE);
        then.status(204);
    });
    client(&server)
        .remove_label(5, "autorelease: pending")
        .unwrap();
    delete.assert_hits(0);
}

#[test]
fn remove_label_not_attached_is_ok() {
    let server = httpmock::MockServer::start();
    label_list(&server, json!([{"id": 3, "name": "autorelease: pending"}]));
    server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("{REPO_PATH}/issues/5/labels/3"));
        then.status(404);
    });
    client(&server)
        .remove_label(5, "autorelease: pending")
        .unwrap();
}
