//! httpmock tests for the Gitea client, split per endpoint group.

mod labels;
mod pulls;
mod releases;

use super::Gitea;
use crate::ForgeKind;
use crate::test_util::config;
use httpmock::MockServer;

const REPO_PATH: &str = "/api/v1/repos/afe/crater";

fn client(server: &MockServer) -> Gitea {
    Gitea::new(config(ForgeKind::Gitea, &server.base_url(), "afe/crater"))
}
