# Logging & Output

Soroban-Profiler produces two distinct kinds of output, and it's important to keep them separate.

## 1. The cost protocol (`harness/`)

Every `record()` call prints exactly one line to stdout:

```
##SOROBAN_COST_JSON##{"label":"transfer","cpu_instructions":69441,"memory_bytes":28032}
```

This is a **machine-readable protocol**, not a log line. It must:

- Stay on a single line (no pretty-printed JSON).
- Keep the `##SOROBAN_COST_JSON##` prefix stable — the CLI's parser matches on it exactly.
- Only ever add fields, never rename or remove existing ones without a major version bump (see `RELEASING.md`).

Do not add any other harness output to stdout. If you need to add debug output for harness development, print to stderr instead, so it doesn't get mistaken for a cost record by the CLI's parser.

## 2. CLI diagnostics (`cli/`)

Everything the CLI prints *other than* the parsed report table:

- Progress messages ("running `cargo test` in examples/token-example ...") go to stdout, matching the example output shown in the README.
- Errors (failed to spawn `cargo test`, manifest not found, no cost records collected) go to stderr with a non-zero exit code.

## Verbosity

There's currently no `--verbose`/`--quiet` flag. If you're adding one:

- `--quiet` should suppress the progress messages but still print the final report table.
- `--verbose` should show the raw `cargo test` output alongside the parsed report, useful for debugging why records aren't showing up.

## Debugging "no records collected"

Run the target's tests directly to see raw output, bypassing the CLI's parsing:

```bash
cd path/to/your/contract
cargo test -- --nocapture | grep SOROBAN_COST_JSON
```

If nothing shows up, the issue is in the target project's test setup (see `docs/runbook.md`), not in the CLI.
