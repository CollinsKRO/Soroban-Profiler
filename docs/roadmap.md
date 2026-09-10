# Roadmap

## v0.1 (current)

Measures CPU instructions and memory bytes — the two dimensions `env.cost_estimate().budget()` exposes directly and reliably across SDK versions.

## v0.2 (planned)

Additional dimensions from `env.cost_estimate().resources()`:

- Ledger entry reads / writes
- Transaction size
- Events size

Blocked on confirming the exact field names on `InvocationResources` for the pinned SDK version (currently `soroban-sdk` 25). Tracking issue: TBD — link here once filed.

## Ideas under consideration (not yet scheduled)

- `--json` output flag for machine-readable CI integration (see issue #1).
- `--filter <name>` to run only specific `record()`-labeled tests, to speed up iteration on large test suites (see `docs/performance.md`).
- Network selection flag (`--network testnet|futurenet|mainnet`) so limits comparisons aren't hardcoded to mainnet — addresses the confusion noted in `docs/runbook.md`.
- Regression detection: compare a report against a previous run (e.g. stored JSON) and flag functions whose cost increased beyond a threshold, useful in CI to catch cost regressions before merge.
- Publishing `soroban-cost-harness` (and possibly `soroban-cost-cli`) to crates.io once the API is stable (see `RELEASING.md`).

## Out of scope (for now)

- Support for non-Soroban chains. This tool is intentionally scoped to Soroban's specific budget/cost model.
- A hosted/SaaS version of the report viewer — the HTML report is designed to be self-contained and offline-first instead.

## How to propose additions

Open an issue describing the use case, not just the feature. See `CONTRIBUTING.md` for the process.
