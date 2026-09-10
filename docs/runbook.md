# Runbook

Operational notes for maintainers — what to do when things go wrong, both in CI and for users filing issues.

## CI failures

### `cargo test --workspace` fails on `examples/token-example`

This is the canary for "did we break the harness or the JSON protocol." Check:

1. Did a recent harness change alter the `##SOROBAN_COST_JSON##` line format without updating `report.rs`'s parser?
2. Did the pinned `soroban-sdk` version in `examples/token-example/Cargo.toml` drift, changing the shape of `env.cost_estimate()`?

### `cargo clippy -- -D warnings` fails after a dependency bump

Usually a new clippy lint on an existing dependency version bump rather than a real bug. Confirm locally with `cargo clippy --workspace`, fix or `#[allow]` with a comment explaining why, don't silently bump the CI config to ignore it.

## Recurring user-reported issues

### "My function shows 0 cost / no record printed"

Almost always one of:
- `record()` wasn't actually called (typo, wrong test, conditional skipped).
- The user is running `cargo test` themselves without `--nocapture` and expecting the CLI's output — point them at running via `soroban-cost-cli report`, which handles this.
- SDK version mismatch — `env.cost_estimate().budget()` field names changed between SDK versions the harness doesn't support yet.

Ask for their `soroban-sdk` version and a minimal repro before debugging further.

### "Percentages look wrong / don't match my own budget() call"

Check whether they're comparing against `limits.rs`'s mainnet defaults when their contract was tested on testnet/futurenet, which may have different limits. This is a common source of confusion — consider whether the CLI should print which network's limits it's using more prominently (see `docs/roadmap.md`).

### Mainnet limits are stale

Soroban's resource limits are periodically adjusted by network vote. If a user reports the limits in `limits.rs` don't match current mainnet, verify against the [Stellar resource limits docs](https://developers.stellar.org/docs/networks/resource-limits-fees) and cut a patch release (see `RELEASING.md`).

## Triage priority

See [docs/triage.md](triage.md) for how incoming issues are labeled and prioritized.
