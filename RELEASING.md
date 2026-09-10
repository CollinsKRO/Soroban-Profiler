# Releasing Soroban-Profiler

This doc covers how versions of `soroban-cost-cli` and `soroban-cost-harness` are cut and published.

## Versioning

Both crates share a workspace version, bumped together, following SemVer:

- **Patch** — bug fixes, doc updates, no behavior change to `record()` output or CLI flags.
- **Minor** — new flags, new report dimensions, backwards-compatible additions (e.g. v0.2's ledger read/write tracking).
- **Major** — breaking changes to the `record()` API, the `##SOROBAN_COST_JSON##` line format, or CLI flag semantics.

## Pre-release checklist

1. `cargo test --workspace` passes.
2. `cargo clippy --workspace -- -D warnings` is clean.
3. `examples/token-example` still profiles correctly end-to-end:
   ```bash
   cargo run -p cli -- report --manifest-path examples/token-example
   ```
4. `Cargo.toml` version bumped in both `cli/` and `harness/`.
5. Changelog entry added (see below).
6. Mainnet limits in `cli/src/limits.rs` checked against the [current Stellar resource limits docs](https://developers.stellar.org/docs/networks/resource-limits-fees) — bump if the network has voted in changes since the last release.

## Changelog

We keep a running `CHANGELOG.md` at the repo root (Keep a Changelog format). Add an entry under `[Unreleased]` as part of your PR if the change is user-facing; the maintainer will move it under the version heading at release time.

## Cutting a release

```bash
git checkout main
git pull
# bump versions in cli/Cargo.toml and harness/Cargo.toml
cargo build --workspace
git commit -am "chore: release vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

Pushing a `v*` tag triggers the release workflow in `.github/workflows`, which builds release binaries and publishes them to the GitHub Releases page.

## Publishing to crates.io

Once the tool is stable enough for a 1.0, `harness/` (and optionally `cli/`) will be published to crates.io so projects can depend on `soroban-cost-harness` without a git dependency. Until then, the README's `git = "..."` dev-dependency instructions are the supported install path.

## Post-release

- Verify the release binary runs against `examples/token-example`.
- Announce in the repo's Discussions tab (or wherever the project's community lives) if the release includes user-facing changes.
