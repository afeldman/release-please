//! Inserting a new entry above the existing ones.

const HEADER: &str = "# Changelog";

/// Insert `entry` before the first `## ` line (after the header), or after
/// the existing header/text if there are no entries yet. Creates the
/// `# Changelog` header if the file has no header.
pub fn prepend(existing: &str, entry: &str) -> String {
    let (head, tail) = match first_entry_offset(existing) {
        Some(index) => existing.split_at(index),
        None => (existing, ""),
    };
    let head = head_with_header(head.trim());
    let mut out = format!("{head}\n\n{}\n", entry.trim_end());
    if !tail.is_empty() {
        out.push('\n');
        out.push_str(tail);
    }
    out
}

/// Byte offset of the first line starting with `## `.
fn first_entry_offset(text: &str) -> Option<usize> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.starts_with("## ") {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

fn head_with_header(head: &str) -> String {
    if head.is_empty() {
        HEADER.to_string()
    } else if head.starts_with("# ") {
        head.to_string()
    } else {
        format!("{HEADER}\n\n{head}")
    }
}
