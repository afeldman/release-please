//! Label changes on merge requests. GitLab creates missing labels on
//! attach, so labels travel as `add_labels` / `remove_labels` on the MR
//! update call.

use super::Gitlab;
use anyhow::Result;
use serde_json::json;

pub(super) fn add_labels(g: &Gitlab, number: u64, labels: &[&str]) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let path = format!("{}/merge_requests/{number}", g.project_path());
    let req = g.api.request("PUT", &path);
    g.api
        .check(
            "add labels",
            req.send_json(json!({"add_labels": labels.join(",")})),
        )
        .map(|_| ())
}

pub(super) fn remove_label(g: &Gitlab, number: u64, label: &str) -> Result<()> {
    let path = format!("{}/merge_requests/{number}", g.project_path());
    let req = g.api.request("PUT", &path);
    g.api
        .check(
            "remove label",
            req.send_json(json!({"remove_labels": label})),
        )
        .map(|_| ())
}
