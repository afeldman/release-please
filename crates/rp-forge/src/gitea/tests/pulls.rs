use super::{REPO_PATH, client};
use crate::Forge;
use crate::test_util::{filler_prs, rest_pr};
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn find_open_pr_pages_until_branches_match() {
    let server = MockServer::start();
    let page1 = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("state", "open")
            .query_param("limit", "50")
            .query_param("page", "1")
            .header("Authorization", "token test-token");
        then.status(200).json_body(json!(filler_prs(50)));
    });
    let page2 = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("state", "open")
            .query_param("page", "2");
        then.status(200)
            .json_body(json!([rest_pr(9, "feat", "main", &[], false)]));
    });
    let pr = client(&server)
        .find_open_pr("feat", "main")
        .unwrap()
        .expect("open PR");
    assert_eq!(pr.number, 9);
    assert_eq!(pr.head_branch, "feat");
    page1.assert_hits(1);
    page2.assert_hits(1);
}

#[test]
fn find_open_pr_none_when_base_differs() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(format!("{REPO_PATH}/pulls"));
        then.status(200)
            .json_body(json!([rest_pr(4, "feat", "dev", &[], false)]));
    });
    assert!(
        client(&server)
            .find_open_pr("feat", "main")
            .unwrap()
            .is_none()
    );
}

#[test]
fn create_pr_posts_payload_and_maps_response() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/pulls"))
            .header("Authorization", "token test-token")
            .json_body(json!({
                "head": "release-please--branches--main",
                "base": "main",
                "title": "chore: release 1.2.0",
                "body": "notes"
            }));
        then.status(201).json_body(rest_pr(
            5,
            "release-please--branches--main",
            "main",
            &["autorelease: pending"],
            false,
        ));
    });
    let pr = client(&server)
        .create_pr(
            "release-please--branches--main",
            "main",
            "chore: release 1.2.0",
            "notes",
        )
        .unwrap();
    assert_eq!(pr.number, 5);
    assert_eq!(pr.labels, vec!["autorelease: pending".to_owned()]);
    mock.assert_hits(1);
}

#[test]
fn update_pr_patches_title_and_body() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("PATCH")
            .path(format!("{REPO_PATH}/pulls/5"))
            .json_body(json!({"title": "new title", "body": "new body"}));
        then.status(200)
            .json_body(rest_pr(5, "h", "main", &[], false));
    });
    client(&server)
        .update_pr(5, "new title", "new body")
        .unwrap();
    mock.assert_hits(1);
}

#[test]
fn merged_prs_filter_by_state_base_and_label() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("state", "closed")
            .query_param("sort", "recentupdate")
            .query_param("page", "1");
        then.status(200).json_body(json!([
            rest_pr(1, "a", "main", &["autorelease: pending"], true),
            rest_pr(2, "b", "main", &["autorelease: pending"], false),
            rest_pr(3, "c", "dev", &["autorelease: pending"], true),
            rest_pr(4, "d", "main", &[], true),
        ]));
    });
    let prs = client(&server)
        .merged_prs_with_label("main", "autorelease: pending")
        .unwrap();
    let numbers: Vec<u64> = prs.iter().map(|p| p.number).collect();
    assert_eq!(numbers, vec![1]);
    assert_eq!(prs[0].merge_sha.as_deref(), Some("abc123"));
    mock.assert_hits(1);
}

#[test]
fn merged_prs_stop_at_fifty() {
    let server = MockServer::start();
    let full: Vec<_> = (1..=50)
        .map(|n| rest_pr(n, "h", "main", &["autorelease: pending"], true))
        .collect();
    let page1 = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("page", "1");
        then.status(200).json_body(json!(full));
    });
    let page2 = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("page", "2");
        then.status(200).json_body(json!([]));
    });
    let prs = client(&server)
        .merged_prs_with_label("main", "autorelease: pending")
        .unwrap();
    assert_eq!(prs.len(), 50);
    page1.assert_hits(1);
    page2.assert_hits(0);
}

#[test]
fn list_error_reports_status_without_token() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(format!("{REPO_PATH}/pulls"));
        then.status(500).body("internal");
    });
    let err = client(&server)
        .find_open_pr("feat", "main")
        .unwrap_err()
        .to_string();
    assert!(err.contains("HTTP 500"), "{err}");
    assert!(!err.contains("test-token"), "{err}");
}
