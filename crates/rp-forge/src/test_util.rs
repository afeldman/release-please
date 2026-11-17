//! Helpers shared by the forge tests (no network, fake tokens only).

use crate::{ForgeConfig, ForgeKind};
use serde_json::{Value, json};

pub(crate) const TOKEN: &str = "test-token";

pub(crate) fn config(kind: ForgeKind, url: &str, repo: &str) -> ForgeConfig {
    ForgeConfig {
        kind,
        url: url.to_owned(),
        repo: repo.to_owned(),
        token: TOKEN.to_owned(),
    }
}

/// Gitea/GitHub pull request object with the given head/base and labels.
pub(crate) fn rest_pr(number: u64, head: &str, base: &str, labels: &[&str], merged: bool) -> Value {
    let labels: Vec<Value> = labels
        .iter()
        .map(|n| json!({"id": number, "name": n}))
        .collect();
    json!({
        "number": number,
        "title": format!("PR {number}"),
        "body": "notes",
        "head": {"ref": head},
        "base": {"ref": base},
        "labels": labels,
        "merged_at": if merged { json!("2026-11-16T10:00:00Z") } else { Value::Null },
        "merge_commit_sha": if merged { json!("abc123") } else { Value::Null },
    })
}

/// `count` unmatched pull requests, used to fill a full page.
pub(crate) fn filler_prs(count: u64) -> Vec<Value> {
    (1000..1000 + count)
        .map(|n| rest_pr(n, "other", "main", &[], false))
        .collect()
}
