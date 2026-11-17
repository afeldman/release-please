//! Dotted key paths from extra-file configs, e.g. `$.package.version`.

/// A parsed `jsonpath` from an extra-file entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPath {
    segments: Vec<String>,
}

impl KeyPath {
    /// Strips a leading `$.` (or a lone `$`) and splits on dots.
    pub fn parse(raw: &str) -> Self {
        let trimmed = raw.strip_prefix("$.").unwrap_or(raw);
        let trimmed = if trimmed == "$" { "" } else { trimmed };
        let segments = trimmed.split('.').map(str::to_string).collect();
        Self { segments }
    }

    pub fn refs(&self) -> Vec<&str> {
        self.segments.iter().map(String::as_str).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::KeyPath;

    #[test]
    fn strips_root_prefix() {
        assert_eq!(
            KeyPath::parse("$.package.version").refs(),
            vec!["package", "version"]
        );
        assert_eq!(KeyPath::parse("version").refs(), vec!["version"]);
    }
}
