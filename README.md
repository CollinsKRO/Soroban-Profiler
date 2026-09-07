# Soroban-Profiler

A Rust-native cost profiler for Soroban smart contracts. Runs your test suite, breaks resource usage down per function, and flags what's close to mainnet's network limits.

## Why

Soroban contracts are billed per-transaction against fixed network resource limits (CPU instructions, memory, disk I/O, tx size, events). The SDK exposes `env.cost_estimate().budget()` for manual inspection, but it only reports **CPU instructions** and **memory bytes** for the entire budget — there is no built-in way to:

- Measure individual function costs in isolation
- Compare usage against real mainnet limits
- See a ranked report of which functions are closest to hitting the ceiling

Soroban-Profiler fills that gap. You wrap each function call in a test harness, run the CLI, and get a per-function breakdown with percentages against current mainnet limits, colored by severity.

## Install

```sh
cargo install --path cli
```

Or run directly from the repo:

```sh
cargo run -p cli -- report --manifest-path path/to/your/contract
```

## Usage

### 1. Add the harness as a dev-dependency

```toml
[dev-dependencies]
soroban-cost-harness = { git = "https://github.com/CollinsKRO/Soroban-Profiler" }
```

### 2. Wrap function calls in `record()`

```rust
#[cfg(test)]
mod tests {
    use soroban_cost_harness::record;

    #[test]
    fn measure_transfer() {
        let env = Env::default();
        let contract_id = env.register(Token, ());
        let client = TokenClient::new(&env, &contract_id);

        // record() measures the closure, prints a JSON cost line to stdout
        record(&env, "transfer", || {
            client.transfer(&from, &to, 100);
        });
    }
}
```

`record()` resets the budget to unlimited before each call so every invocation measures only the cost of the wrapped function in isolation.

### 3. Run the profiler

```sh
# Terminal report (colored table, sorted worst-first)
soroban-cost-cli report --manifest-path path/to/your/contract

# Also write a self-contained HTML report
soroban-cost-cli report --manifest-path path/to/your/contract --html report.html
```

### Example output

```
=> running `cargo test` in examples/token-example ...

=> collected 4 cost record(s):

  label             cpu       cpu %           mem       mem %
  batch_transfer    199609     0.2%           75223     0.2%
  swap               69443     0.1%           28040     0.1%
  transfer           69441     0.1%           28032     0.1%
  mint               38101     0.0%           17151     0.0%
```

Colors: green (< 60%), yellow (60-85%), red (> 85%) of the mainnet limit.

The included [`examples/token-example`](examples/token-example) demonstrates all four profiled functions. `batch_transfer` deliberately re-reads the same storage key in a loop to show up as the worst offender.

## How it works

1. The CLI runs `cargo test -- --nocapture` in the target manifest directory
2. Test output lines prefixed with `##SOROBAN_COST_JSON##` are parsed as `{label, cpu_instructions, memory_bytes}` records
3. Records are compared against current mainnet limits and displayed as a sorted, color-coded table
4. Optionally, a self-contained HTML report is written (inline CSS, no external assets, opens offline)

## Mainnet resource limits (v0.1 defaults)

| Resource | Limit |
|---|---|
| CPU instructions | 100,000,000 |
| Memory | 40 MB |
| Disk read | 200 KB |
| Disk write | 132 KiB |
| Tx size | 132 KB |
| Events return | 16 KB |

These values are periodically adjusted by network vote. See [Stellar resource limits docs](https://developers.stellar.org/docs/networks/resource-limits-fees) for the latest.

## Scope

### v0.1 (current)

Measures **CPU instructions** and **memory bytes** — the two dimensions `env.cost_estimate().budget()` exposes directly and reliably across SDK versions.

### v0.2 (planned)

Additional dimensions from `env.cost_estimate().resources()`:

- Ledger entry reads / writes
- Transaction size
- Events size

These require confirming the exact field names on `InvocationResources` for the pinned SDK version (currently soroban-sdk 25).

## Project structure

```
├── cli/                          # soroban-cost-cli binary
│   └── src/
│       ├── main.rs               # CLI entry point, cargo test runner
│       ├── report.rs             # Terminal report (colored table)
│       ├── html_report.rs        # Self-contained HTML report
│       └── limits.rs             # Mainnet SorobanLimits
├── harness/                      # soroban-cost-harness library
│   └── src/lib.rs                # record() function + CostRecord
└── examples/
    └── token-example/            # Example contract with profiled tests
```

## License

MIT
