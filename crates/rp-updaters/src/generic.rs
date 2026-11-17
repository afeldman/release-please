//! Generic release-please markers, usable in any text file:
//!
//! - `x-release-please-version` on a line: first semver on that line.
//! - `x-release-please-start-version` ... `x-release-please-end`: first semver
//!   of every line in between.
//! - `x-release-please-major|minor|patch` on a line: only that component of
//!   the first semver on the line is replaced.
//!
//! Marker lines without a semver are an error; everything else is untouched.

use crate::Updater;
use anyhow::{Result, bail};
use regex::Regex;
use semver::Version;

const VERSION: &str = "x-release-please-version";
const START: &str = "x-release-please-start-version";
const END: &str = "x-release-please-end";
const SEMVER: &str = r"(\d+)\.(\d+)\.(\d+)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?";

#[derive(Debug, Clone, Copy, Default)]
pub struct GenericMarkers;

impl Updater for GenericMarkers {
    fn update(&self, content: &str, version: &Version) -> Result<String> {
        let re = Regex::new(SEMVER)?;
        let mut in_block = false;
        let mut out = String::with_capacity(content.len());
        for line in content.split_inclusive('\n') {
            if line.contains(START) {
                in_block = true;
            } else if line.contains(END) {
                in_block = false;
            }
            if line.contains(START) || line.contains(END) {
                out.push_str(line);
                continue;
            }
            out.push_str(&edit_line(line, &re, version, in_block)?);
        }
        Ok(out)
    }
}

fn edit_line(line: &str, re: &Regex, version: &Version, in_block: bool) -> Result<String> {
    let component = component_marker(line);
    let marked = component.is_some() || line.contains(VERSION);
    if !marked && !in_block {
        return Ok(line.to_string());
    }
    let edited = match component {
        Some(idx) => replace_component(line, re, idx, version),
        None => replace_whole(line, re, version),
    };
    match edited {
        Some(text) => Ok(text),
        None if marked => bail!("release-please marker without a semver: {}", line.trim()),
        None => Ok(line.to_string()),
    }
}

/// Index of the version component named by a major/minor/patch marker.
fn component_marker(line: &str) -> Option<usize> {
    if line.contains("x-release-please-major") {
        Some(0)
    } else if line.contains("x-release-please-minor") {
        Some(1)
    } else if line.contains("x-release-please-patch") {
        Some(2)
    } else {
        None
    }
}

fn replace_whole(line: &str, re: &Regex, version: &Version) -> Option<String> {
    let m = re.find(line)?;
    Some(format!(
        "{}{}{}",
        &line[..m.start()],
        version,
        &line[m.end()..]
    ))
}

fn replace_component(line: &str, re: &Regex, idx: usize, version: &Version) -> Option<String> {
    let caps = re.captures(line)?;
    let group = caps.get(idx + 1)?;
    let value = match idx {
        0 => version.major,
        1 => version.minor,
        _ => version.patch,
    };
    Some(format!(
        "{}{}{}",
        &line[..group.start()],
        value,
        &line[group.end()..]
    ))
}
