# Security Policy

## Scope

Soroban-Profiler is a developer-side tooling project: a CLI and test harness that runs `cargo test` against a target contract and parses cost output. It does not run against mainnet, does not hold funds, and does not execute arbitrary contract code beyond what the user's own test suite already does.

That said, we take the following seriously, since the tool does run untrusted-ish input (a target crate's test suite) and produces reports that teams may use to make deployment decisions:

- Vulnerabilities that let a malicious `Cargo.toml`/test suite escape the intended `cargo test` sandboxing in a way that's specific to this tool (beyond what `cargo test` itself already allows).
- Bugs in `limits.rs` or the cost-comparison logic that could cause a contract to be reported as safely under a mainnet resource limit when it is not (a false negative in the red/yellow/green severity classification).
- Parsing bugs in the `##SOROBAN_COST_JSON##` line format that could be exploited to inject misleading report data.

## Supported versions

| Version | Supported |
| ------- | --------- |
| Latest `main` | ✅ |
| Tagged releases | ✅ (most recent) |
| Older tags | ❌ |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security reports.

Instead, use GitHub's private vulnerability reporting: go to the repo's **Security** tab → **Report a vulnerability**. If that's not available, email the maintainer listed on the [GitHub profile](https://github.com/CollinsKRO) with a subject line starting `[SECURITY]`.

Please include:

- A description of the issue and its impact (e.g. "causes a red-severity function to be misreported as green").
- Steps to reproduce, ideally a minimal `record()` call or test crate.
- The `soroban-sdk` and Rust toolchain versions you used.

## What to expect

- Acknowledgment within a few days.
- We'll work with you on a fix and coordinate disclosure timing before any public write-up.
- Credit in the release notes, if you'd like it.

## Out of scope

- Issues in `soroban-sdk` itself, or in the Soroban/Stellar network's actual resource limits — report those upstream to the [Stellar Development Foundation](https://developers.stellar.org).
- General cost-accuracy discrepancies that aren't security-relevant (open a normal bug report for those).
