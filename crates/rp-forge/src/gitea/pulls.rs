//! Pull request endpoints of the Gitea API.

use super::Gitea;
use crate::PullRequest;
use crate::pages;
use crate::rest_pr::RestPr;
use anyhow::Result;
use serde_json::json;

const PER_PAGE: usize = 50;
const MERGED_LIMIT: usize = 50;

fn list_pulls(g: &Gitea, state: &str, sort: Option<&str>, page: u32) -> Result<Vec<RestPr>> {
    let mut req = g
        .api
        .request("GET", &format!("{}/pulls", g.repo_path()))
        .query("state", state)
        .query("limit", &PER_PAGE.to_string())
        .query("page", &page.to_string());
    if let Some(sort) = sort {
        req = req.query("sort", sort);
    }
    let resp = g.api.check("list pull requests", req.call())?;
    Ok(resp.into_json()?)
}

pub(super) fn find_open_pr(g: &Gitea, head: &str, base: &str) -> Result<Option<PullRequest>> {
    let mut found = None;
    pages::walk(
        PER_PAGE,
        |page| list_pulls(g, "open", None, page),
        |pr| {
            if pr.matches_branches(head, base) {
                found = Some(pr.into_pr());
                false
            } else {
                true
            }
        },
    )?;
    Ok(found)
}

pub(super) fn create_pr(
    g: &Gitea,
    head: &str,
    base: &str,
    title: &str,
    body: &str,
) -> Result<PullRequest> {
    let payload = json!({"head": head, "base": base, "title": title, "body": body});
    let req = g.api.request("POST", &format!("{}/pulls", g.repo_path()));
    let resp = g.api.check("create pull request", req.send_json(payload))?;
    Ok(resp.into_json::<RestPr>()?.into_pr())
}

pub(super) fn update_pr(g: &Gitea, number: u64, title: &str, body: &str) -> Result<()> {
    let path = format!("{}/pulls/{number}", g.repo_path());
    let req = g.api.request("PATCH", &path);
    g.api
        .check(
            "update pull request",
            req.send_json(json!({"title": title, "body": body})),
        )
        .map(|_| ())
}

pub(super) fn merged_prs_with_label(
    g: &Gitea,
    base: &str,
    label: &str,
) -> Result<Vec<PullRequest>> {
    let mut out = Vec::new();
    pages::walk(
        PER_PAGE,
        |page| list_pulls(g, "closed", Some("recentupdate"), page),
        |pr| {
            if pr.is_merged_into_with(base, label) {
                out.push(pr.into_pr());
            }
            out.len() < MERGED_LIMIT
        },
    )?;
    Ok(out)
}
