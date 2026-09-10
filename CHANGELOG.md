# Changelog

All notable changes to this project are documented here. Format loosely follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

- Added `CONTRIBUTING.md`, `SECURITY.md`, `RELEASING.md`, `CODE_OF_CONDUCT.md`.
- Added `docs/` with architecture, local-setup, runbook, performance, triage, logging, and roadmap docs.
- Added issue templates and a PR template under `.github/`.
- Added `scripts/install.sh` and `scripts/run-example.sh` helper scripts.

## [0.1.0]

- Initial release: `soroban-cost-cli` and `soroban-cost-harness`, measuring CPU instructions and memory bytes per function.
- Colored terminal report, sorted worst-first.
- Optional self-contained HTML report via `--html`.
- `examples/token-example` demonstrating all four profiled functions.
