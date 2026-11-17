//! httpmock tests for the GitHub client, split per endpoint group.

mod labels;
mod pulls;
mod releases;

use super::Github;
use crate::ForgeKind;
use crate::test_util::config;
use httpmock::MockServer;

const REPO_PATH: &str = "/repos/afe/crater";

/// `url` is the API root, so the server base URL is used as-is.
fn client(server: &MockServer) -> Github {
    Github::new(config(ForgeKind::Github, &server.base_url(), "afe/crater"))
}
