use super::{REPO_PATH, client};
use crate::Forge;
use crate::test_util::{filler_prs, rest_pr};
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn find_open_pr_filters_by_owner_head_and_base() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("state", "open")
            .query_param("head", "afe:release-please--branches--main")
            .query_param("base", "main");
        then.status(200).json_body(json!([rest_pr(
            12,
            "release-please--branches--main",
            "main",
            &["autorelease: pending"],
            false
        )]));
    });
    let pr = client(&server)
        .find_open_pr("release-please--branches--main", "main")
        .unwrap()
        .expect("open PR");
    assert_eq!(pr.number, 12);
    assert_eq!(pr.labels, vec!["autorelease: pending".to_owned()]);
    mock.assert_hits(1);
}

#[test]
fn find_open_pr_none_when_list_is_empty() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(format!("{REPO_PATH}/pulls"));
        then.status(200).json_body(json!(filler_prs(3)));
    });
    assert!(
        client(&server)
            .find_open_pr("feat", "main")
            .unwrap()
            .is_none()
    );
}

#[test]
fn sends_bearer_and_api_version_headers() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .header("Authorization", "Bearer test-token")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header_exists("User-Agent");
        then.status(200).json_body(json!([]));
    });
    assert!(
        client(&server)
            .find_open_pr("feat", "main")
            .unwrap()
            .is_none()
    );
    mock.assert_hits(1);
}

#[test]
fn create_pr_posts_payload_and_maps_response() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{REPO_PATH}/pulls"))
            .json_body(json!({
                "head": "feat",
                "base": "main",
                "title": "feat: thing",
                "body": "notes"
            }));
        then.status(201)
            .json_body(rest_pr(21, "feat", "main", &[], false));
    });
    let pr = client(&server)
        .create_pr("feat", "main", "feat: thing", "notes")
        .unwrap();
    assert_eq!(pr.number, 21);
    assert_eq!(pr.head_branch, "feat");
    mock.assert_hits(1);
}

#[test]
fn update_pr_patches_title_and_body() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method("PATCH")
            .path(format!("{REPO_PATH}/pulls/21"))
            .json_body(json!({"title": "t2", "body": "b2"}));
        then.status(200)
            .json_body(rest_pr(21, "feat", "main", &[], false));
    });
    client(&server).update_pr(21, "t2", "b2").unwrap();
    mock.assert_hits(1);
}

#[test]
fn merged_prs_query_closed_and_filter_merged_base_label() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{REPO_PATH}/pulls"))
            .query_param("state", "closed")
            .query_param("base", "main")
            .query_param("sort", "updated")
            .query_param("direction", "desc")
            .query_param("page", "1");
        then.status(200).json_body(json!([
            rest_pr(30, "a", "main", &["autorelease: pending"], true),
            rest_pr(31, "b", "main", &["autorelease: pending"], false),
            rest_pr(32, "c", "main", &[], true),
            rest_pr(33, "d", "main", &["autorelease: pending"], true),
        ]));
    });
    let prs = client(&server)
        .merged_prs_with_label("main", "autorelease: pending")
        .unwrap();
    let numbers: Vec<u64> = prs.iter().map(|p| p.number).collect();
    assert_eq!(numbers, vec![30, 33]);
    mock.assert_hits(1);
}
