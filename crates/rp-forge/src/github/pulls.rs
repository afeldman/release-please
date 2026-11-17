//! Pull request endpoints of the GitHub REST API.

use super::Github;
use crate::PullRequest;
use crate::pages;
use crate::rest_pr::RestPr;
use anyhow::Result;
use serde_json::json;

const PER_PAGE: usize = 50;
const MERGED_LIMIT: usize = 50;

fn list_pulls(g: &Github, state: &str, extra: &[(&str, &str)], page: u32) -> Result<Vec<RestPr>> {
    let mut req = g
        .api
        .request("GET", &format!("{}/pulls", g.repo_path()))
        .query("state", state)
        .query("per_page", &PER_PAGE.to_string())
        .query("page", &page.to_string());
    for (key, value) in extra {
        req = req.query(key, value);
    }
    let resp = g.api.check("list pull requests", req.call())?;
    Ok(resp.into_json()?)
}

pub(super) fn find_open_pr(g: &Github, head: &str, base: &str) -> Result<Option<PullRequest>> {
    let owner_head = format!("{}:{head}", g.owner);
    let extra = [("head", owner_head.as_str()), ("base", base)];
    let prs = list_pulls(g, "open", &extra, 1)?;
    Ok(prs
        .into_iter()
        .find(|pr| pr.matches_branches(head, base))
        .map(RestPr::into_pr))
}

pub(super) fn create_pr(
    g: &Github,
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

pub(super) fn update_pr(g: &Github, number: u64, title: &str, body: &str) -> Result<()> {
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
    g: &Github,
    base: &str,
    label: &str,
) -> Result<Vec<PullRequest>> {
    let extra = [("base", base), ("sort", "updated"), ("direction", "desc")];
    let mut out = Vec::new();
    pages::walk(
        PER_PAGE,
        |page| list_pulls(g, "closed", &extra, page),
        |pr| {
            if pr.is_merged_into_with(base, label) {
                out.push(pr.into_pr());
            }
            out.len() < MERGED_LIMIT
        },
    )?;
    Ok(out)
}
