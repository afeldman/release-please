//! httpmock tests for the GitLab client, split per endpoint group.

mod labels;
mod mrs;
mod releases;

use super::Gitlab;
use crate::ForgeKind;
use crate::test_util::config;
use httpmock::MockServer;
use serde_json::{Value, json};

/// Project path with `/` encoded, as the GitLab API expects.
const PROJECT_PATH: &str = "/api/v4/projects/afe%2Fcrater";

fn client(server: &MockServer) -> Gitlab {
    Gitlab::new(config(ForgeKind::Gitlab, &server.base_url(), "afe/crater"))
}

/// Merge request object; `state` is "opened" or "merged".
pub(super) fn mr(iid: u64, labels: &[&str], target: &str, state: &str) -> Value {
    json!({
        "iid": iid,
        "title": format!("MR {iid}"),
        "description": null,
        "source_branch": "feat",
        "target_branch": target,
        "labels": labels,
        "state": state,
        "merge_commit_sha": null,
        "squash_commit_sha": null,
    })
}
