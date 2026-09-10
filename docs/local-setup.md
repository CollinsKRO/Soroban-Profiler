# Local Setup

## Prerequisites

- Rust toolchain (stable), installed via [rustup](https://rustup.rs).
- `cargo` on your `PATH`.
- A Soroban-compatible target contract to test against, if you're developing against something other than `examples/token-example`.

## Clone and build

```bash
git clone https://github.com/CollinsKRO/Soroban-Profiler.git
cd Soroban-Profiler
cargo build --workspace
```

## Run the test suite

```bash
cargo test --workspace
```

## Try it against the bundled example

```bash
cargo run -p cli -- report --manifest-path examples/token-example
```

You should see a colored table with `batch_transfer`, `swap`, `transfer`, and `mint` rows, matching the example output in the README.

To also generate an HTML report:

```bash
cargo run -p cli -- report --manifest-path examples/token-example --html report.html
open report.html   # or `xdg-open` on Linux
```

## Using the harness in your own contract (for manual testing during development)

Point a local path dependency at your checkout instead of the git dependency shown in the README:

```toml
[dev-dependencies]
soroban-cost-harness = { path = "../Soroban-Profiler/harness" }
```

Then wrap a test call in `record()` as shown in the README's usage section, and run the CLI's `report` command with `--manifest-path` pointing at your contract.

## Linting and formatting

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

These are the same checks run in CI (`.github/workflows`) — run them locally before opening a PR to avoid a round-trip.

## Common issues

- **"no cost records collected"** — make sure your tests call `record()` and that you're not accidentally running `cargo test` without `--nocapture` somewhere in a custom test runner; the CLI runs this for you, but if you're testing the harness manually, output capturing will swallow the JSON lines.
- **Version mismatch with `soroban-sdk`** — the harness reads fields via `env.cost_estimate()`, which has changed shape across SDK versions. Check the SDK version pinned in `examples/token-example/Cargo.toml` against your own project's.
