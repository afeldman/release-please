//! Percent-encoding for URL path segments.

/// Encodes one path segment. Everything outside the RFC 3986 unreserved set
/// (including `/`) is escaped.
pub(crate) fn segment(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Encodes `owner/name` segment by segment, keeping the separating `/`.
pub(crate) fn path(s: &str) -> String {
    s.split('/').map(segment).collect::<Vec<_>>().join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_escapes_colon_and_space() {
        assert_eq!(segment("autorelease: pending"), "autorelease%3A%20pending");
    }

    #[test]
    fn segment_escapes_slash_for_gitlab_project_ids() {
        assert_eq!(segment("group/sub/proj"), "group%2Fsub%2Fproj");
    }

    #[test]
    fn path_keeps_separator_between_segments() {
        assert_eq!(path("owner/my repo"), "owner/my%20repo");
    }
}
