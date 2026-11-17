# release-please (Rust)

Automated releases from conventional commits for **Gitea/Forgejo, GitHub and GitLab**.

This project rethinks [googleapis/release-please](https://github.com/googleapis/release-please)
in Rust. It keeps the release-please workflow and configuration format and
opens it up to more forges. It is an independent reimplementation, not a fork,
and is not affiliated with or endorsed by Google.

## Why

- **More forges.** release-please talks only to the GitHub API. This version
  puts a small `Forge` trait in front of the API and ships with Gitea (which also
  covers Forgejo), GitHub and GitLab.
- **One static binary.** There is no Node runtime and no dependency churn. It runs
  as a CLI, as a Woodpecker plugin, as a GitHub Actions step or as a GitLab CI job.
- **Compatible configuration.** It reads `release-please-config.json` and
  `.release-please-manifest.json`, a documented subset of the upstream format.
- **Formatting-preserving edits.** It edits `Cargo.toml`/`Cargo.lock`, `pyproject.toml`,
  `package.json`, `Chart.yaml`, `version.txt` and files with release-please markers.
  Each edit changes only the version and leaves comments, order and indentation alone.
- **Customizable.** You can configure changelog sections, tag format, pre-1.0 bump rules,
  prerelease channels (`-rc.N`) and a changelog template (minijinja).

## Workflow

1. `release-please-rs release-pr` runs on every push to the base branch.
   It computes the next versions, updates the version files and the changelog,
   and opens or updates a release pull request.
2. You merge the release pull request.
3. `release-please-rs release` tags each package and creates the forge releases.

`release-please-rs plan` shows what would happen without writing anything.

## Status

Early development. See [DESIGN.md](DESIGN.md).

## License

Apache License 2.0, see [LICENSE](LICENSE) and [NOTICE](NOTICE).
The workflow and configuration format originate from
[googleapis/release-please](https://github.com/googleapis/release-please)
(Copyright Google LLC, Apache-2.0).
