# REPO.md

A native SSH client for the Linux desktop: a Rust GTK application plus separate documentation. It is one platform of the Bastion family and has its own versions.

## Stack

- Rust workspace in `LinuxApp/` (the app crate plus the `integrations` member crate).
- GTK 4 UI with libadwaita, a VTE terminal and GtkSourceView.
- SSH through russh, async runtime tokio.
- Documentation in `docs/`, built with Zensical (`zensical.toml`).
- GitHub Actions for docs, release and wiki sync; CodeQL for analysis; Renovate for dependencies.

## Commands

Run from `LinuxApp/` (the build dependencies in AGENTS.md must be installed):

```bash
cargo build              # works
cargo clippy --workspace # runs, but currently reports warnings; not a gate
cargo fmt --check        # runs, but the tree is not formatted; not a gate
```

Not run here: the docs build (`zensical`, only in CI) and `packaging/build-rpm.sh` (needs a Fedora container).

## Where things live

- `LinuxApp/integrations/` has no dependencies on purpose, so it cannot reach GTK, russh or the filesystem. Keep it that way.
- `LinuxApp/src/*_ui.rs` hold the GTK parts of a feature; the matching file without the suffix holds the logic.
- `.github/release-baseline` and `.github/release-required-checks` steer the release workflow.
- `docs/agents/` holds issue-tracker and domain notes for agents.

## Release

Merging to `main` releases automatically. The PR title (Conventional Commits) sets the version; see `docs/release.md`. A release publishes the docs but nothing to a package manager or store. The RPM is built separately.

## Repo-specific rules

- No CI job builds or tests the app, so the local test run is the only gate.
- `rust-version` in `LinuxApp/Cargo.toml` is a verified floor; raise it only after rerunning the tests on that toolchain.
