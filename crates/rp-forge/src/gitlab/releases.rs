//! Release endpoints of the GitLab API.
//! GitLab has no draft releases and no prerelease flag: `draft` is rejected,
//! `prerelease` is ignored.

use super::Gitlab;
use crate::NewRelease;
use crate::encode;
use anyhow::{Result, bail};
use serde_json::json;

pub(super) fn create_release(g: &Gitlab, release: &NewRelease) -> Result<()> {
    if release.draft {
        bail!("GitLab does not support draft releases");
    }
    let payload = json!({
        "tag_name": release.tag,
        "ref": release.target_sha,
        "name": release.name,
        "description": release.body,
    });
    let req = g
        .api
        .request("POST", &format!("{}/releases", g.project_path()));
    g.api
        .check("create release", req.send_json(payload))
        .map(|_| ())
}

pub(super) fn release_exists(g: &Gitlab, tag: &str) -> Result<bool> {
    let path = format!("{}/releases/{}", g.project_path(), encode::segment(tag));
    let req = g.api.request("GET", &path);
    Ok(g.api
        .check_found("get release by tag", req.call())?
        .is_some())
}
