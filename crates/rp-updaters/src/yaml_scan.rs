//! Line-based edit of one scalar in block-style YAML.
//!
//! Limitations: keys are matched as plain text (quoted keys are not
//! recognised), flow-style mappings are not supported, and only plain or
//! single/double-quoted scalars are rewritten. Everything else in the file,
//! including comments and indentation, is copied byte for byte.

/// Replaces the scalar at a key path (e.g. `["image", "tag"]`) with `new`.
/// Returns `None` if the key is missing or its value is not a scalar.
pub fn set_scalar(content: &str, path: &[&str], new: &str) -> Option<String> {
    let lines: Vec<&str> = content.split_inclusive('\n').collect();
    let (last, parents) = path.split_last()?;
    let mut range = 0..lines.len();
    let mut parent: Option<usize> = None;
    for key in parents {
        let hit = find_key(&lines, range, parent, key)?;
        range = hit.line + 1..lines.len();
        parent = Some(hit.indent);
    }
    let hit = find_key(&lines, range, parent, last)?;
    let edited = replace_value(lines[hit.line], hit.colon_end, new)?;
    let mut out = String::with_capacity(content.len() + new.len());
    for (idx, line) in lines.iter().enumerate() {
        if idx == hit.line {
            out.push_str(&edited);
        } else {
            out.push_str(line);
        }
    }
    Some(out)
}

struct Hit {
    line: usize,
    indent: usize,
    /// Byte offset just after the colon, within the line.
    colon_end: usize,
}

/// Finds `key:` among the direct children of `parent` inside `range`.
/// The first content line in the range fixes the child indentation; a line
/// indented at or below the parent's indentation ends the block.
fn find_key(
    lines: &[&str],
    range: std::ops::Range<usize>,
    parent: Option<usize>,
    key: &str,
) -> Option<Hit> {
    let mut child: Option<usize> = None;
    for idx in range {
        let body = lines[idx].trim_end_matches(['\r', '\n']);
        let trimmed = body.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let indent = body.len() - trimmed.len();
        if parent.is_some_and(|p| indent <= p) {
            break;
        }
        match child {
            None => child = Some(indent),
            Some(c) if indent < c => break,
            Some(c) if indent > c => continue,
            Some(_) => {}
        }
        if let Some(colon) = colon_after(trimmed, key) {
            return Some(Hit {
                line: idx,
                indent,
                colon_end: indent + colon,
            });
        }
    }
    None
}

/// Offset just past `key:` at the start of `trimmed`, if the line is that key.
fn colon_after(trimmed: &str, key: &str) -> Option<usize> {
    let rest = trimmed.strip_prefix(key)?;
    let after = rest.trim_start_matches([' ', '\t']);
    let gap = rest.len() - after.len();
    after.starts_with(':').then_some(key.len() + gap + 1)
}

/// Rewrites the scalar that follows `from` on `line`, keeping quotes and comments.
fn replace_value(line: &str, from: usize, new: &str) -> Option<String> {
    let rest = &line[from..];
    let value_start = from + (rest.len() - rest.trim_start_matches([' ', '\t']).len());
    let first = *line.as_bytes().get(value_start)?;
    let (start, end) = if first == b'"' || first == b'\'' {
        let close = line[value_start + 1..].find(first as char)? + value_start + 1;
        (value_start + 1, close)
    } else {
        if matches!(first, b'|' | b'>' | b'[' | b'{' | b'&' | b'*' | b'#') {
            return None;
        }
        let len = line[value_start..]
            .find([' ', '\t', '#', '\r', '\n'])
            .unwrap_or(line.len() - value_start);
        if len == 0 {
            return None;
        }
        (value_start, value_start + len)
    };
    Some(format!("{}{}{}", &line[..start], new, &line[end..]))
}
