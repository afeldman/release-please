# release-please-rs: design

Rust reimplementation of the release-please workflow for Gitea/Forgejo,
GitHub and GitLab. Small, static binary; usable as CLI and Woodpecker /
GitHub Actions / GitLab CI step.

## Flow

1. `release-pr` (on every push to the base branch)
   - per package: last release = highest tag matching the package
     (`rp_core::tag::parse_tag`), else manifest version, else none
   - commits since that tag touching the package path -> conventional
     commits -> next version (`rp_core::version::next_version`)
   - edit files (`rp-updaters`), prepend changelog (`rp-changelog`),
     write `.release-please-manifest.json`
   - commit on `release-please--branches--<base>` (optionally signed),
     force-with-lease push, open/update PR, label `autorelease: pending`
2. `release` (on every push to the base branch, after merge)
   - merged PRs with `autorelease: pending`: for every package whose
     manifest version has no release, create tag + release with notes
     from the changelog, then relabel `autorelease: tagged`
3. `plan`: dry run of step 1, human or `--json` output.

## Crates

| crate | job |
|---|---|
| rp-core | config (release-please JSON subset), manifest, commit parsing, bump rules, tags |
| rp-updaters | format-preserving edits: Cargo.toml/lock, pyproject, package.json/lock, Chart.yaml, version.txt, generic markers, json/toml/yaml keys |
| rp-changelog | markdown entries, optional minijinja template, extract notes |
| rp-git | git CLI wrapper (honours signing, credentials, hooks) |
| rp-forge | `Forge` trait; Gitea (also Forgejo), GitHub, GitLab REST via ureq |
| rp-cli | clap binary `release-please-rs`, orchestration |

## Rules

- Files <= ~150 lines; split modules instead of growing files.
- Tests need no network: httpmock for forges, temp git repos for rp-git.
- Formatting of user files must survive updates byte-for-byte except the
  version string.
- Generic marker (release-please compatible): a line containing
  `x-release-please-version` gets its semver replaced; lines between
  `x-release-please-start-version` and `x-release-please-end` too.
