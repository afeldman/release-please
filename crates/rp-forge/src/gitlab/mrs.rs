//! Merge request endpoints of the GitLab API.

use super::Gitlab;
use crate::PullRequest;
use crate::pages;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

const PER_PAGE: usize = 50;
const MERGED_LIMIT: usize = 50;

#[derive(Debug, Deserialize)]
pub(super) struct Mr {
    pub iid: u64,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    pub source_branch: String,
    pub target_branch: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub state: String,
    #[serde(default)]
    pub merge_commit_sha: Option<String>,
    #[serde(default)]
    pub squash_commit_sha: Option<String>,
}

impl Mr {
    fn is_merged(&self) -> bool {
        self.state == "merged"
    }

    fn into_pr(self) -> PullRequest {
        let merge_sha = if self.is_merged() {
            self.merge_commit_sha.or(self.squash_commit_sha)
        } else {
            None
        };
        PullRequest {
            number: self.iid,
            title: self.title,
            body: self.description.unwrap_or_default(),
            head_branch: self.source_branch,
            base_branch: self.target_branch,
            labels: self.labels,
            merge_sha,
        }
    }
}

fn list_mrs(g: &Gitlab, extra: &[(&str, &str)], page: u32) -> Result<Vec<Mr>> {
    let mut req = g
        .api
        .request("GET", &format!("{}/merge_requests", g.project_path()))
        .query("per_page", &PER_PAGE.to_string())
        .query("page", &page.to_string());
    for (key, value) in extra {
        req = req.query(key, value);
    }
    let resp = g.api.check("list merge requests", req.call())?;
    Ok(resp.into_json()?)
}

pub(super) fn find_open_pr(g: &Gitlab, head: &str, base: &str) -> Result<Option<PullRequest>> {
    let extra = [
        ("state", "opened"),
        ("source_branch", head),
        ("target_branch", base),
    ];
    let mrs = list_mrs(g, &extra, 1)?;
    Ok(mrs
        .into_iter()
        .find(|mr| mr.source_branch == head && mr.target_branch == base)
        .map(Mr::into_pr))
}

pub(super) fn create_pr(
    g: &Gitlab,
    head: &str,
    base: &str,
    title: &str,
    body: &str,
) -> Result<PullRequest> {
    let payload = json!({
        "source_branch": head,
        "target_branch": base,
        "title": title,
        "description": body,
    });
    let req = g
        .api
        .request("POST", &format!("{}/merge_requests", g.project_path()));
    let resp = g
        .api
        .check("create merge request", req.send_json(payload))?;
    Ok(resp.into_json::<Mr>()?.into_pr())
}

pub(super) fn update_pr(g: &Gitlab, number: u64, title: &str, body: &str) -> Result<()> {
    let path = format!("{}/merge_requests/{number}", g.project_path());
    let req = g.api.request("PUT", &path);
    g.api
        .check(
            "update merge request",
            req.send_json(json!({"title": title, "description": body})),
        )
        .map(|_| ())
}

pub(super) fn merged_prs_with_label(
    g: &Gitlab,
    base: &str,
    label: &str,
) -> Result<Vec<PullRequest>> {
    let extra = [
        ("state", "merged"),
        ("target_branch", base),
        ("labels", label),
        ("order_by", "updated_at"),
        ("sort", "desc"),
    ];
    let mut out = Vec::new();
    pages::walk(
        PER_PAGE,
        |page| list_mrs(g, &extra, page),
        |mr| {
            if mr.is_merged() && mr.target_branch == base && mr.labels.iter().any(|l| l == label) {
                out.push(mr.into_pr());
            }
            out.len() < MERGED_LIMIT
        },
    )?;
    Ok(out)
}
