//! Byte-level JSON scanner that finds one string value by key path.
//!
//! Only the value's bytes are rewritten, so indentation, key order, spacing
//! and everything else stay identical. Keys are compared raw (no unescaping).
//! For duplicate keys the first occurrence wins.

use std::ops::Range;

/// Replaces the string value at `path` (object keys from the root) with `new`.
/// Returns `None` if the path is missing or the value is not a string.
pub fn set_string(content: &str, path: &[&str], new: &str) -> Option<String> {
    let span = string_span(content, path)?;
    let mut out = String::with_capacity(content.len() + new.len() + 2);
    out.push_str(&content[..span.start]);
    out.push('"');
    out.push_str(new);
    out.push('"');
    out.push_str(&content[span.end..]);
    Some(out)
}

/// Byte span (including the quotes) of the string value at `path`.
pub fn string_span(content: &str, path: &[&str]) -> Option<Range<usize>> {
    if path.is_empty() {
        return None;
    }
    let mut scan = Scanner {
        src: content.as_bytes(),
        pos: 0,
    };
    scan.find(path)
}

struct Scanner<'a> {
    src: &'a [u8],
    pos: usize,
}

impl Scanner<'_> {
    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    /// Searches the object at the cursor for `path`, descending into nested objects.
    fn find(&mut self, path: &[&str]) -> Option<Range<usize>> {
        let (target, rest) = path.split_first()?;
        self.skip_ws();
        if self.peek()? != b'{' {
            return None;
        }
        self.pos += 1;
        loop {
            self.skip_ws();
            if self.peek()? == b'}' {
                return None;
            }
            let key = self.string()?;
            self.skip_ws();
            if self.peek()? != b':' {
                return None;
            }
            self.pos += 1;
            self.skip_ws();
            if &self.src[key.start + 1..key.end - 1] == target.as_bytes() {
                return if rest.is_empty() {
                    self.string()
                } else {
                    self.find(rest)
                };
            }
            self.skip_value()?;
            self.skip_ws();
            if self.peek()? != b',' {
                return None;
            }
            self.pos += 1;
        }
    }

    /// Reads a string token at the cursor and returns its span with quotes.
    fn string(&mut self) -> Option<Range<usize>> {
        if self.peek()? != b'"' {
            return None;
        }
        let start = self.pos;
        self.pos += 1;
        loop {
            match self.peek()? {
                b'\\' => self.pos += 2,
                b'"' => {
                    self.pos += 1;
                    return Some(start..self.pos);
                }
                _ => self.pos += 1,
            }
        }
    }

    fn skip_value(&mut self) -> Option<()> {
        match self.peek()? {
            b'"' => {
                self.string()?;
            }
            b'{' | b'[' => self.skip_nested()?,
            _ => {
                while let Some(b) = self.peek() {
                    if matches!(b, b',' | b'}' | b']' | b' ' | b'\t' | b'\r' | b'\n') {
                        break;
                    }
                    self.pos += 1;
                }
            }
        }
        Some(())
    }

    fn skip_nested(&mut self) -> Option<()> {
        let mut depth = 0usize;
        loop {
            match self.peek()? {
                b'"' => {
                    self.string()?;
                    continue;
                }
                b'{' | b'[' => depth += 1,
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => {}
            }
            self.pos += 1;
            if depth == 0 {
                return Some(());
            }
        }
    }
}
