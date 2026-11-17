//! Commands:
//! - `plan`: show next versions and file edits (no writes).
//! - `release-pr`: update files on `release-please--branches--<base>`,
//!   push, open/update the release PR with the pending label.
//! - `release`: for merged PRs with the pending label, tag + create
//!   releases for each package whose manifest version has no release yet,
//!   then swap the label to the tagged label.
//! Every option also reads `PLUGIN_*` (Woodpecker) and `RP_*` env vars.

fn main() -> anyhow::Result<()> {
    todo!()
}
