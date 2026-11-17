//! Inheritance: top-level options are defaults, `packages` entries override
//! them per path.

use super::{PackageConfig, RawPackages, default_sections};
use anyhow::{Context, Result, bail};
use serde_json::map::Entry;
use serde_json::{Map, Value};

/// Resolve every package. Without a `packages` map the top-level options
/// describe the single root package (`.`).
pub(super) fn resolve_packages(root: &Map<String, Value>) -> Result<Vec<PackageConfig>> {
    let mut defaults = root.clone();
    let overrides: RawPackages = match defaults.remove("packages") {
        None | Some(Value::Null) => RawPackages::new(),
        Some(Value::Object(map)) => map.into_iter().collect(),
        Some(_) => bail!("`packages` must be an object keyed by path"),
    };
    if overrides.is_empty() {
        return Ok(vec![resolve_one(".", Value::Object(defaults))?]);
    }
    overrides
        .into_iter()
        .map(|(path, over)| {
            let mut merged = Value::Object(defaults.clone());
            deep_merge(&mut merged, over);
            resolve_one(&path, merged)
        })
        .collect()
}

/// Merge `over` into `base`: objects merge key by key, every other value
/// (including arrays) is replaced.
pub(super) fn deep_merge(base: &mut Value, over: Value) {
    match (base, over) {
        (Value::Object(base_map), Value::Object(over_map)) => {
            for (key, value) in over_map {
                match base_map.entry(key) {
                    Entry::Occupied(mut slot) => deep_merge(slot.get_mut(), value),
                    Entry::Vacant(slot) => {
                        slot.insert(value);
                    }
                }
            }
        }
        (slot, over) => *slot = over,
    }
}

fn resolve_one(path: &str, value: Value) -> Result<PackageConfig> {
    let mut pkg: PackageConfig = serde_json::from_value(value)
        .with_context(|| format!("invalid options for package {path:?}"))?;
    pkg.path = path.to_string();
    if pkg.changelog_sections.is_none() {
        pkg.changelog_sections = Some(default_sections());
    }
    Ok(pkg)
}

/// Read a top-level string option, or `default` when absent or not a string.
pub(super) fn string_option(root: &Map<String, Value>, key: &str, default: &str) -> String {
    root.get(key)
        .and_then(Value::as_str)
        .unwrap_or(default)
        .to_string()
}

/// Read a top-level boolean option; absent or non-boolean means `false`.
pub(super) fn bool_option(root: &Map<String, Value>, key: &str) -> bool {
    root.get(key).and_then(Value::as_bool).unwrap_or(false)
}
