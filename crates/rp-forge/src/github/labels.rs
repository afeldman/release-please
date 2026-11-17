//! Label endpoints of the GitHub REST API. Adding a label that does not
//! exist yet creates it, so no separate create step is needed.

use super::Github;
use crate::encode;
use anyhow::Result;
use serde_json::json;

pub(super) fn add_labels(g: &Github, number: u64, labels: &[&str]) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let path = format!("{}/issues/{number}/labels", g.repo_path());
    let req = g.api.request("POST", &path);
    g.api
        .check("add labels", req.send_json(json!({"labels": labels})))
        .map(|_| ())
}

/// Removing a label that is not attached to the PR is a no-op (404).
pub(super) fn remove_label(g: &Github, number: u64, label: &str) -> Result<()> {
    let path = format!(
        "{}/issues/{number}/labels/{}",
        g.repo_path(),
        encode::segment(label)
    );
    let req = g.api.request("DELETE", &path);
    g.api.check_found("remove label", req.call())?;
    Ok(())
}
