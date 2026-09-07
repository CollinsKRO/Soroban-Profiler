# Soroban-Profiler

A Rust-native cost profiler for Soroban smart contracts. Runs your test suite, breaks resource usage down per function, and flags what's close to mainnet's network limits.

## Why

Soroban contracts are billed per-transaction against fixed network resource limits. The host metering system tracks **five** resource dimensions per invocation — CPU instructions, memory, ledger reads, ledger writes, and event size — but the SDK's `env.cost_estimate().budget()` only exposes CPU and memory. There is no built-in way to:

- See all five resource dimensions for an individual function
- Compare each dimension against real mainnet limits
- Get a ranked report of which functions are closest to hitting the ceiling

Soroban-Profiler fills that gap. You wrap each function call in a test harness, run the CLI, and get a per-function breakdown across all five dimensions with percentages against current mainnet limits, colored by severity.

## Install

```sh
cargo install soroban-cost-cli
```

### Build from source (contributors)

```sh
git clone https://github.com/CollinsKRO/Soroban-Profiler.git
cd Soroban-Profiler
cargo install --path cli
```

## Usage

### 1. Add the harness as a dev-dependency

```toml
[dev-dependencies]
soroban-cost-harness = "0.1"
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

  label           read bytes     rd %      wr bytes     wr %       events     evt %
  batch_transfer           0     0.0%           896     0.7%           0     0.0%
  swap                     0     0.0%           512     0.4%           0     0.0%
  transfer                 0     0.0%           512     0.4%           0     0.0%
  mint                     0     0.0%           256     0.2%           0     0.0%
```

Colors: green (< 60%), yellow (60-85%), red (> 85%) of the mainnet limit.

### Real-world output: Stellar Liquidity Pool

Profiled against [stellar/soroban-examples](https://github.com/stellar/soroban-examples)
at tag `v23.0.0` (commit `b46f4e0`) — the official constant-product AMM contract.

```
=> running `cargo test` in /tmp/profile-liquidity-pool ...

=> collected 3 cost record(s):

  label         cpu       cpu %           mem       mem %
  deposit     586171       0.6%         85616       0.2%
  swap        555912       0.6%         78750       0.2%
  withdraw    591875       0.6%         83071       0.2%

  label       read bytes     rd %      wr bytes     wr %       events     evt %
  deposit            184     0.1%         1504     1.1%          472     2.9%
  swap                 0     0.0%         1360     1.0%          472     2.9%
  withdraw             0     0.0%         1504     1.1%          472     2.9%
```

All three functions are well within network limits. The `deposit` and `withdraw`
operations are the most expensive (~586K–592K CPU instructions) due to token
transfers and share accounting. `swap` is slightly cheaper at ~556K instructions.
Event size is constant across all three at 472 bytes (2.9% of the 16 KB limit).

## How it works

1. The CLI runs `cargo test -- --nocapture` in the target manifest directory
2. Test output lines prefixed with `##SOROBAN_COST_JSON##` are parsed as cost records containing all five resource dimensions
3. Records are compared against current mainnet limits and displayed as a sorted, color-coded two-section table (compute + I/O)
4. Optionally, a self-contained HTML report is written (inline CSS, no external assets, opens offline)

## What is measured

Each `record()` call captures five resource dimensions from the Soroban host's `InvocationResources`:

| Dimension | Source field | Description |
|---|---|---|
| CPU instructions | `instructions` | Modelled instruction count |
| Memory bytes | `mem_bytes` | Peak memory usage |
| Ledger read bytes | `disk_read_bytes` | Bytes read from disk (restorations, classic entries) |
| Ledger write bytes | `write_bytes` | Bytes written to the ledger |
| Events size bytes | `contract_events_size_bytes` | Total size of emitted contract events |

**Note:** Transaction size is not included because the host's `InvocationResources` struct explicitly excludes it — tx size depends on XDR serialization which is not modelled in the test environment.

## Mainnet resource limits

| Resource | Limit |
|---|---|
| CPU instructions | 100,000,000 |
| Memory | 40 MB |
| Disk read | 200 KB |
| Disk write | 132 KiB |
| Events return | 16 KB |

These values are periodically adjusted by network vote. See [Stellar resource limits docs](https://developers.stellar.org/docs/networks/resource-limits-fees) for the latest.

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
