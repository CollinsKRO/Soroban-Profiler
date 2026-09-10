# Architecture

## Overview

Soroban-Profiler has two crates and one example, wired together by a simple text protocol:

```
 your contract's tests
        │
        │  record(&env, "label", || { ... })
        ▼
┌─────────────────────┐
│ soroban-cost-harness │  resets budget → runs closure → reads env.cost_estimate().budget()
└──────────┬───────────┘  prints "##SOROBAN_COST_JSON##{...}" to stdout
           │
           │  cargo test -- --nocapture
           ▼
┌─────────────────────┐
│  soroban-cost-cli    │  spawns `cargo test`, scrapes stdout for the JSON marker lines
└──────────┬───────────┘
           │
           ├─► report.rs      → colored terminal table, sorted worst-first
           ├─► html_report.rs → self-contained offline HTML report
           └─► limits.rs      → mainnet SorobanLimits used for % calculations
```

## Why a stdout protocol instead of a library call?

The CLI runs the target project's *own* `cargo test`, in the target project's own process. The harness has no way to hand structured data back to the CLI process directly — there's no shared memory, and the CLI isn't linked against the target crate. Printing a marked JSON line to stdout and parsing it back out is simple, doesn't require the target crate to add any CLI-specific dependency beyond the harness itself, and survives `cargo test`'s normal output buffering when run with `--nocapture`.

## Component responsibilities

### `harness/` — `soroban-cost-harness`

- `record(env, label, f)`: resets the environment's budget to unlimited, runs `f`, reads back CPU instructions and memory bytes from `env.cost_estimate().budget()`, and prints one `##SOROBAN_COST_JSON##{...}` line per call.
- Intentionally tiny — it should be safe to add as a dev-dependency without pulling in extra build weight.

### `cli/` — `soroban-cost-cli`

- `main.rs` — argument parsing, spawns `cargo test -- --nocapture` in the target manifest directory, captures stdout.
- `report.rs` — parses the JSON lines, sorts by resource usage (worst first), renders a colored terminal table.
- `html_report.rs` — same data, rendered as a single self-contained HTML file (inline CSS, no external assets) via `--html`.
- `limits.rs` — hardcoded mainnet `SorobanLimits` (CPU, memory, disk I/O, tx size, events) used to compute the percentage-of-limit and severity color for each function.

### `examples/token-example/`

A minimal token contract with `record()` calls wired into its tests. Used both as a smoke test for CI and as the source of the example output in the README. `batch_transfer` is deliberately inefficient (re-reads the same storage key in a loop) so the example report has a visible "red" entry.

## Data flow: one `record()` call

1. Harness resets the env's budget to unlimited so the measurement isn't polluted by setup cost from earlier assertions in the same test.
2. The wrapped closure runs.
3. Harness reads `env.cost_estimate().budget()` for CPU instructions and memory bytes.
4. Harness prints `##SOROBAN_COST_JSON##{"label":"...","cpu_instructions":N,"memory_bytes":N}`.
5. CLI's `cargo test` subprocess captures this line among normal test output.
6. CLI filters lines by the `##SOROBAN_COST_JSON##` prefix, deserializes the rest as JSON.
7. Each record is compared against `limits.rs` to compute percentage and severity, then sorted and rendered.

## Extension points (see `docs/roadmap.md`)

The v0.2 plan (ledger reads/writes, tx size, events) slots into the same pipeline: `harness` reads more fields from `env.cost_estimate().resources()`, adds them to the JSON payload, and `report.rs`/`html_report.rs` render additional columns. `limits.rs` already has placeholder limits for these dimensions.
