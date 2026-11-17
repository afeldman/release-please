//! Shared blocking HTTP helper: base URL, auth headers, error mapping.
//! Error messages carry the method-independent `what` label, the HTTP
//! status and a truncated body. The token is never included.

use anyhow::{Result, anyhow};
use std::time::Duration;

const MAX_BODY_CHARS: usize = 300;
const TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct Api {
    agent: ureq::Agent,
    base: String,
    token: String,
    headers: Vec<(&'static str, String)>,
}

impl Api {
    /// `base` is the API root (trailing slash is trimmed); `headers` are
    /// attached to every request.
    pub(crate) fn new(base: &str, token: &str, headers: Vec<(&'static str, String)>) -> Self {
        Self {
            agent: ureq::AgentBuilder::new().timeout(TIMEOUT).build(),
            base: base.trim_end_matches('/').to_owned(),
            token: token.to_owned(),
            headers,
        }
    }

    /// Builds a request for `path` (starting with `/`) with the auth headers set.
    pub(crate) fn request(&self, method: &str, path: &str) -> ureq::Request {
        let mut req = self
            .agent
            .request(method, &format!("{}{}", self.base, path));
        for (name, value) in &self.headers {
            req = req.set(name, value);
        }
        req
    }

    /// Any non-2xx status (including 404) becomes an error labelled `what`.
    pub(crate) fn check(
        &self,
        what: &str,
        res: std::result::Result<ureq::Response, ureq::Error>,
    ) -> Result<ureq::Response> {
        res.map_err(|err| self.describe(what, err))
    }

    /// 404 maps to `Ok(None)`; every other failure is an error.
    pub(crate) fn check_found(
        &self,
        what: &str,
        res: std::result::Result<ureq::Response, ureq::Error>,
    ) -> Result<Option<ureq::Response>> {
        match res {
            Ok(resp) => Ok(Some(resp)),
            Err(ureq::Error::Status(404, _)) => Ok(None),
            Err(err) => Err(self.describe(what, err)),
        }
    }

    fn describe(&self, what: &str, err: ureq::Error) -> anyhow::Error {
        match err {
            ureq::Error::Status(code, resp) => {
                let body = resp.into_string().unwrap_or_default();
                let body = self.redact(&body);
                let short: String = body.chars().take(MAX_BODY_CHARS).collect();
                anyhow!("{what}: HTTP {code}: {short}")
            }
            ureq::Error::Transport(err) => anyhow!("{what}: transport error: {err}"),
        }
    }

    fn redact(&self, text: &str) -> String {
        if self.token.is_empty() {
            text.to_owned()
        } else {
            text.replace(&self.token, "[redacted]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn error_contains_status_and_body_but_not_token() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/x");
            then.status(500).body("server said test-token is bad");
        });
        let api = Api::new(&server.base_url(), "test-token", vec![]);
        let err = api
            .check("probe", api.request("GET", "/x").call())
            .unwrap_err()
            .to_string();
        assert!(err.contains("probe"), "{err}");
        assert!(err.contains("HTTP 500"), "{err}");
        assert!(err.contains("[redacted]"), "{err}");
        assert!(!err.contains("test-token"), "{err}");
    }

    #[test]
    fn check_found_maps_404_to_none() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/missing");
            then.status(404);
        });
        let api = Api::new(&server.base_url(), "t", vec![]);
        let found = api.check_found("probe", api.request("GET", "/missing").call());
        assert!(found.unwrap().is_none());
    }

    #[test]
    fn check_found_keeps_other_errors() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/boom");
            then.status(403);
        });
        let api = Api::new(&server.base_url(), "t", vec![]);
        let res = api.check_found("probe", api.request("GET", "/boom").call());
        assert!(res.unwrap_err().to_string().contains("HTTP 403"));
    }
}
