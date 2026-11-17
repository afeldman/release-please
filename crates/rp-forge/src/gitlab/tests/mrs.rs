use super::{PROJECT_PATH, client, mr};
use crate::Forge;
use httpmock::prelude::*;
use serde_json::json;

#[test]
fn find_open_pr_queries_source_and_target() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/merge_requests"))
            .query_param("state", "opened")
            .query_param("source_branch", "feat")
            .query_param("target_branch", "main")
            .header("PRIVATE-TOKEN", "test-token");
        then.status(200)
            .json_body(json!([mr(7, &["release"], "main", "opened")]));
    });
    let pr = client(&server)
        .find_open_pr("feat", "main")
        .unwrap()
        .expect("open MR");
    assert_eq!(pr.number, 7);
    assert_eq!(pr.head_branch, "feat");
    assert_eq!(pr.base_branch, "main");
    assert_eq!(pr.body, "");
    assert_eq!(pr.labels, vec!["release".to_owned()]);
    mock.assert_hits(1);
}

#[test]
fn find_open_pr_none_when_empty() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/merge_requests"));
        then.status(200).json_body(json!([]));
    });
    assert!(
        client(&server)
            .find_open_pr("feat", "main")
            .unwrap()
            .is_none()
    );
}

#[test]
fn create_pr_posts_mr_and_maps_iid_and_description() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("{PROJECT_PATH}/merge_requests"))
            .json_body(json!({
                "source_branch": "feat",
                "target_branch": "main",
                "title": "feat: thing",
                "description": "notes"
            }));
        let mut body = mr(11, &[], "main", "opened");
        body["description"] = json!("notes");
        then.status(201).json_body(body);
    });
    let pr = client(&server)
        .create_pr("feat", "main", "feat: thing", "notes")
        .unwrap();
    assert_eq!(pr.number, 11);
    assert_eq!(pr.body, "notes");
    mock.assert_hits(1);
}

#[test]
fn update_pr_puts_title_and_description() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path(format!("{PROJECT_PATH}/merge_requests/7"))
            .json_body(json!({"title": "t2", "description": "b2"}));
        then.status(200).json_body(mr(7, &[], "main", "opened"));
    });
    client(&server).update_pr(7, "t2", "b2").unwrap();
    mock.assert_hits(1);
}

#[test]
fn merged_mrs_filter_state_target_and_label() {
    let server = MockServer::start();
    let mut squashed = mr(41, &["autorelease: pending"], "main", "merged");
    squashed["squash_commit_sha"] = json!("sq41");
    let mut merged = mr(40, &["autorelease: pending"], "main", "merged");
    merged["merge_commit_sha"] = json!("mc40");
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/merge_requests"))
            .query_param("state", "merged")
            .query_param("target_branch", "main")
            .query_param("labels", "autorelease: pending")
            .query_param("page", "1");
        then.status(200).json_body(json!([
            merged,
            squashed,
            mr(39, &["autorelease: pending"], "dev", "merged"),
            mr(38, &["autorelease: pending"], "main", "opened"),
        ]));
    });
    let prs = client(&server)
        .merged_prs_with_label("main", "autorelease: pending")
        .unwrap();
    let numbers: Vec<u64> = prs.iter().map(|p| p.number).collect();
    assert_eq!(numbers, vec![40, 41]);
    assert_eq!(prs[0].merge_sha.as_deref(), Some("mc40"));
    assert_eq!(prs[1].merge_sha.as_deref(), Some("sq41"));
    mock.assert_hits(1);
}

#[test]
fn open_mr_has_no_merge_sha_even_if_reported() {
    let server = MockServer::start();
    let mut open = mr(5, &[], "main", "opened");
    open["merge_commit_sha"] = json!("tm5");
    server.mock(|when, then| {
        when.method(GET)
            .path(format!("{PROJECT_PATH}/merge_requests"));
        then.status(200).json_body(json!([open]));
    });
    let pr = client(&server)
        .find_open_pr("feat", "main")
        .unwrap()
        .unwrap();
    assert_eq!(pr.merge_sha, None);
}
