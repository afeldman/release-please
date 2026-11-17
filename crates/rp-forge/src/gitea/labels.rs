//! Label endpoints of the Gitea API. Gitea attaches labels by numeric id,
//! so names are resolved against the repo label list first.

use super::Gitea;
use crate::pages;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

const PER_PAGE: usize = 50;
const NEW_LABEL_COLOR: &str = "#ededed";

#[derive(Debug, Deserialize)]
struct Label {
    id: u64,
    name: String,
}

fn list_labels(g: &Gitea) -> Result<Vec<Label>> {
    let mut all = Vec::new();
    pages::walk(
        PER_PAGE,
        |page| -> Result<Vec<Label>> {
            let req = g
                .api
                .request("GET", &format!("{}/labels", g.repo_path()))
                .query("limit", &PER_PAGE.to_string())
                .query("page", &page.to_string());
            let resp = g.api.check("list labels", req.call())?;
            Ok(resp.into_json()?)
        },
        |label| {
            all.push(label);
            true
        },
    )?;
    Ok(all)
}

fn create_label(g: &Gitea, name: &str) -> Result<u64> {
    let req = g.api.request("POST", &format!("{}/labels", g.repo_path()));
    let payload = json!({"name": name, "color": NEW_LABEL_COLOR});
    let resp = g.api.check("create label", req.send_json(payload))?;
    Ok(resp.into_json::<Label>()?.id)
}

pub(super) fn add_labels(g: &Gitea, number: u64, labels: &[&str]) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let existing = list_labels(g)?;
    let mut ids = Vec::with_capacity(labels.len());
    for name in labels {
        let id = match existing.iter().find(|l| l.name == *name) {
            Some(label) => label.id,
            None => create_label(g, name)?,
        };
        ids.push(id);
    }
    let req = g
        .api
        .request("POST", &format!("{}/issues/{number}/labels", g.repo_path()));
    g.api
        .check("attach labels", req.send_json(json!({"labels": ids})))
        .map(|_| ())
}

/// Removing a label that is not in the repo, or not on the PR, is a no-op.
pub(super) fn remove_label(g: &Gitea, number: u64, label: &str) -> Result<()> {
    let Some(found) = list_labels(g)?.into_iter().find(|l| l.name == label) else {
        return Ok(());
    };
    let path = format!("{}/issues/{number}/labels/{}", g.repo_path(), found.id);
    let req = g.api.request("DELETE", &path);
    g.api.check_found("remove label", req.call())?;
    Ok(())
}
