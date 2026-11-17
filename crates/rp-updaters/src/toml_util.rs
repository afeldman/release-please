//! Shared TOML helpers on `toml_edit`, which keeps formatting on round-trip.

use anyhow::{Result, anyhow};
use toml_edit::{DocumentMut, Item, Value};

/// Parses a TOML document for a format-preserving round-trip.
pub fn parse(content: &str) -> Result<DocumentMut> {
    content
        .parse::<DocumentMut>()
        .map_err(|e| anyhow!("invalid TOML: {e}"))
}

/// Mutable item at a dotted key path (tables and inline tables).
pub fn item_mut<'a>(doc: &'a mut DocumentMut, path: &[&str]) -> Option<&'a mut Item> {
    let (first, rest) = path.split_first()?;
    let mut item = doc.get_mut(first)?;
    for key in rest {
        item = item.as_table_like_mut()?.get_mut(key)?;
    }
    Some(item)
}

/// Replaces a string value in place. The value's surrounding whitespace,
/// trailing comment and quote style (basic or literal) are kept.
/// Returns false if the item is not a string.
pub fn set_string(item: &mut Item, new: &str) -> bool {
    let Some(value) = item.as_value_mut() else {
        return false;
    };
    let literal = match value {
        Value::String(formatted) => formatted.display_repr().starts_with('\''),
        _ => return false,
    };
    let quote = if literal { '\'' } else { '"' };
    let Ok(mut fresh) = format!("{quote}{new}{quote}").parse::<Value>() else {
        return false;
    };
    *fresh.decor_mut() = value.decor().clone();
    *value = fresh;
    true
}

/// Sets the string at `path`. Returns false if missing or not a string.
pub fn set_string_at(doc: &mut DocumentMut, path: &[&str], new: &str) -> bool {
    item_mut(doc, path).is_some_and(|item| set_string(item, new))
}
