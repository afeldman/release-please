//! Release endpoints shared by Gitea and GitHub (`{repo_path}/releases`).

use crate::NewRelease;
use crate::encode;
use crate::http::Api;
use anyhow::Result;
use serde_json::json;

pub(crate) fn create(api: &Api, repo_path: &str, release: &NewRelease) -> Result<()> {
    let payload = json!({
        "tag_name": release.tag,
        "target_commitish": release.target_sha,
        "name": release.name,
        "body": release.body,
        "draft": release.draft,
        "prerelease": release.prerelease,
    });
    let req = api.request("POST", &format!("{repo_path}/releases"));
    api.check("create release", req.send_json(payload))
        .map(|_| ())
}

pub(crate) fn exists(api: &Api, repo_path: &str, tag: &str) -> Result<bool> {
    let path = format!("{repo_path}/releases/tags/{}", encode::segment(tag));
    let req = api.request("GET", &path);
    Ok(api.check_found("get release by tag", req.call())?.is_some())
}
