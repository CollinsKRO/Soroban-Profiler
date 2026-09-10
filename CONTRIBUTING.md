# Contributing to Soroban-Profiler

Thanks for taking the time to contribute! This document covers how to get set up, the workflow we expect for changes, and the standards PRs are held to.

## Before you start

- Check open [issues](https://github.com/CollinsKRO/Soroban-Profiler/issues) — if what you want to do isn't tracked, open an issue first so we can discuss scope before you write code.
- For anything non-trivial (new flags, new resource dimensions, changes to the JSON line protocol), please open an issue proposing the change before submitting a PR.
- Small fixes (typos, docs, obvious bugs) can go straight to a PR.

## Development setup

See [docs/local-setup.md](docs/local-setup.md) for full environment setup. Short version:

```bash
git clone https://github.com/CollinsKRO/Soroban-Profiler.git
cd Soroban-Profiler
cargo build
cargo test --workspace
cargo run -p cli -- report --manifest-path examples/token-example
```

## Project layout

- `cli/` — the `soroban-cost-cli` binary (report generation, table/HTML rendering, mainnet limits).
- `harness/` — the `soroban-cost-harness` library, exposing `record()` for wrapping test calls.
- `examples/token-example/` — a reference contract used for manual testing and in the README.
- `docs/` — architecture, runbook, and process documentation.

## Making a change

1. Fork the repo and create a branch off `main`.
2. Write your change with tests. If you touch `harness/`, add or update a test in `examples/token-example` that exercises the new behavior.
3. Run the full check suite locally before opening a PR:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```
4. Keep PRs focused — one logical change per PR. Large refactors should be discussed in an issue first.
5. Fill out the PR template. Link the issue it closes.

## Commit messages

We don't enforce a strict format, but please write commit subjects that describe the *effect* of the change (e.g. `cli: add --json output flag`) rather than the process (`fix stuff`, `wip`).

## Code style

- Formatting is enforced via `cargo fmt` and linting via `cargo clippy` (see `.github/workflows` for the exact CI checks).
- Public functions and types in `harness/` and `cli/` should have doc comments (`///`) explaining intent, not just restating the signature.
- Avoid `unwrap()`/`expect()` outside of tests and CLI entry points — surface errors with context instead.

## Reporting bugs / requesting features

Use the issue templates under [`.github/ISSUE_TEMPLATE`](.github/ISSUE_TEMPLATE). Include your `soroban-sdk` version and, if relevant, a minimal `record()` call that reproduces the issue.

## Questions

If something in this doc is unclear, open an issue with the `question` label — improving onboarding docs counts as a contribution too.
