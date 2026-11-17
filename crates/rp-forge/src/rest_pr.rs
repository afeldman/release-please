//! Pull request JSON shared by Gitea and GitHub (identical field layout).

use crate::PullRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct RestPr {
    pub number: u64,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    pub head: RestRef,
    pub base: RestRef,
    #[serde(default)]
    pub labels: Vec<RestLabel>,
    #[serde(default)]
    pub merged_at: Option<String>,
    #[serde(default)]
    pub merge_commit_sha: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RestRef {
    #[serde(rename = "ref")]
    pub git_ref: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RestLabel {
    pub name: String,
}

impl RestPr {
    pub(crate) fn is_merged(&self) -> bool {
        self.merged_at.is_some()
    }

    pub(crate) fn has_label(&self, label: &str) -> bool {
        self.labels.iter().any(|l| l.name == label)
    }

    pub(crate) fn matches_branches(&self, head: &str, base: &str) -> bool {
        self.head.git_ref == head && self.base.git_ref == base
    }

    /// Merged into `base` and carrying `label`.
    pub(crate) fn is_merged_into_with(&self, base: &str, label: &str) -> bool {
        self.is_merged() && self.base.git_ref == base && self.has_label(label)
    }

    pub(crate) fn into_pr(self) -> PullRequest {
        let merge_sha = if self.is_merged() {
            self.merge_commit_sha.clone()
        } else {
            None
        };
        PullRequest {
            number: self.number,
            title: self.title,
            body: self.body.unwrap_or_default(),
            head_branch: self.head.git_ref,
            base_branch: self.base.git_ref,
            labels: self.labels.into_iter().map(|l| l.name).collect(),
            merge_sha,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(merged: bool, labels: &[&str]) -> RestPr {
        serde_json::from_value(serde_json::json!({
            "number": 7,
            "title": "feat: x",
            "body": null,
            "head": {"ref": "release-please--branches--main"},
            "base": {"ref": "main"},
            "labels": labels.iter().map(|n| serde_json::json!({"name": n})).collect::<Vec<_>>(),
            "merged_at": if merged { serde_json::json!("2026-11-16T10:00:00Z") } else { serde_json::Value::Null },
            "merge_commit_sha": "abc123",
        }))
        .unwrap()
    }

    #[test]
    fn unmerged_pr_has_no_merge_sha() {
        let pr = sample(false, &[]).into_pr();
        assert_eq!(pr.merge_sha, None);
        assert_eq!(pr.body, "");
        assert_eq!(pr.base_branch, "main");
    }

    #[test]
    fn merged_pr_keeps_merge_sha_and_labels() {
        let pr = sample(true, &["autorelease: pending"]).into_pr();
        assert_eq!(pr.merge_sha.as_deref(), Some("abc123"));
        assert_eq!(pr.labels, vec!["autorelease: pending".to_owned()]);
    }

    #[test]
    fn merged_into_with_checks_state_base_and_label() {
        let pr = sample(true, &["autorelease: pending"]);
        assert!(pr.is_merged_into_with("main", "autorelease: pending"));
        assert!(!pr.is_merged_into_with("dev", "autorelease: pending"));
        assert!(!pr.is_merged_into_with("main", "other"));
        assert!(
            !sample(false, &["autorelease: pending"])
                .is_merged_into_with("main", "autorelease: pending")
        );
    }
}
