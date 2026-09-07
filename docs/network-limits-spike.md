# Network Limits Fetching — Research Spike

## TL;DR

**Feasible.** Soroban RPC's `getLedgerEntries` method exposes all config settings as `CONFIG_SETTING` ledger entries. We can fetch them at runtime and fall back to hardcoded defaults when unavailable.

---

## What We Need

Our profiler measures 6 resource dimensions (CPU, memory, read bytes, write bytes, tx size, events size). Each needs a network-wide limit for the percentage calculations:

| Dimension | Network Config Setting | Ledger Entry Key |
|-----------|----------------------|------------------|
| CPU (instructions) | `tx_max_instructions` | `ContractComputeV0` |
| Memory (bytes) | `tx_memory_limit` | `ContractComputeV0` |
| Ledger read bytes | `tx_max_disk_read_bytes` | `ContractLedgerCostV0` |
| Ledger write bytes | `tx_max_write_bytes` | `ContractLedgerCostV0` |
| Transaction size | `tx_max_size_bytes` | `ContractBandwidthV0` |
| Events size | `tx_max_contract_events_size_bytes` | `ContractEventsV0` |

---

## How It Works

### RPC Method: `getLedgerEntries`

The Stellar/Soroban RPC exposes `getLedgerEntries` which accepts an array of base64-encoded XDR `LedgerKey` values and returns the corresponding ledger entries.

**Request:**
```json
{
  "keys": ["AAAAAAgAAAA=", "AAAAAAgAAAA="]
}
```

**Response:**
```json
{
  "entries": [
    {
      "key": "AAAAAAgAAAA=",
      "xdr": "<base64-encoded ConfigSettingEntry XDR>",
      "lastModifiedLedgerSeq": 12345
    }
  ],
  "latestLedger": 12346
}
```

### XDR Key Construction

Each config setting is a `LedgerKey::ConfigSetting` with a `config_setting_id` discriminant:

| ConfigSettingId | Discriminant | Human Name |
|----------------|-------------|------------|
| ContractComputeV0 | 0 | `CONFIG_SETTING_CONTRACT_COMPUTE_V0` |
| ContractLedgerCostV0 | 1 | `CONFIG_SETTING_CONTRACT_LEDGER_COST_V0` |
| ContractHistoricalDataV0 | 2 | `CONFIG_SETTING_CONTRACT_HISTORICAL_DATA_V0` |
| ContractEventsV0 | 3 | `CONFIG_SETTING_CONTRACT_EVENTS_V0` |
| ContractBandwidthV0 | 4 | `CONFIG_SETTING_CONTRACT_BANDWIDTH_V0` |
| StateArchival | 5 | `CONFIG_SETTING_STATE_ARCHIVAL` |

The XDR key is 8 bytes: 4 bytes for the `LedgerEntryType` discriminant (8 for `ConfigSetting`) + 4 bytes for the `ConfigSettingId` (big-endian u32).

### XDR Response Decoding

The `xdr` field in the response is a base64-encoded `ConfigSettingEntry` XDR struct. The relevant variants:

```rust
// From stellar_xdr
enum ConfigSettingEntry {
    ContractCompute(ContractComputeV0),      // id=0
    ContractLedgerCost(ContractLedgerCostV0), // id=1
    ContractHistoricalData(ContractHistoricalDataV0), // id=2
    ContractEvents(ContractEventsV0),         // id=3
    ContractBandwidth(ContractBandwidthV0),   // id=4
    StateArchival(StateArchivalSettings),     // id=5
}

struct ContractComputeV0 {
    tx_max_instructions: u64,
    tx_memory_limit: u32,
}

struct ContractLedgerCostV0 {
    tx_max_read_ledger_entries: u32,
    tx_max_write_ledger_entries: u32,
    tx_max_read_bytes: u32,
    tx_max_write_bytes: u32,
    tx_fee_read_ledger_entry: i64,
    tx_fee_write_ledger_entry: i64,
    tx_fee_read1_kb: i64,
    tx_fee_write1_kb: i64,
    contract_data_size_limit: u32,
}

struct ContractEventsV0 {
    tx_max_contract_events_size_bytes: u32,
}

struct ContractBandwidthV0 {
    tx_max_size_bytes: u32,
}
```

---

## Feasibility Assessment

### ✅ What Works

1. **RPC endpoint availability** — Every Soroban network (mainnet, testnet, futurenet) runs a Stellar RPC that supports `getLedgerEntries`
2. **Config settings are public** — They're ledger entries, not contract data, so no auth required
3. **Batched fetch** — All 6 settings can be fetched in a single RPC call (pass 6 keys)
4. **Existing implementations** — `soroban-cost-estimator` crate (by `aigbagbobila`) already does this successfully
5. **XDR types available** — `stellar-xdr` crate provides full type definitions

### ⚠️ Challenges

1. **Async runtime** — RPC calls require `tokio` or similar async runtime
2. **XDR version coupling** — `stellar-xdr` version must match the network's protocol version; mismatched versions may fail to decode
3. **Network dependency** — Requires a reachable RPC endpoint; no offline fallback without hardcoded defaults
4. **Rate limiting** — Public RPCs have rate limits; caching is advisable

### ❌ Not Feasible (Yet)

1. **Runtime fetching inside `soroban-env-host`** — The Wasm environment doesn't expose RPC calls; fetching must happen in the CLI, not in the test harness
2. **Per-invocation limits** — The config settings are *per-transaction*, not per-invocation. If a transaction calls multiple contracts, limits apply to the transaction, not each call. Our profiler measures per-function, so we'd be comparing against per-transaction limits (conservative/overestimate for single-call transactions)

---

## Recommended Approach

### Architecture

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Test Harness │────▶│   CLI        │────▶│  RPC Client  │
│  (record())   │     │  (parse JSON)│     │  (fetch limits)│
└──────────────┘     └──────────────┘     └──────────────┘
                                            │
                                     ┌──────▼──────┐
                                     │  Hardcoded  │
                                     │  Fallback   │
                                     └─────────────┘
```

### Implementation Plan

**Phase 1: Add `--rpc-url` flag to CLI**
- Add `--rpc-url <URL>` flag to `soroban-cost-cli report`
- When provided, fetch config settings before rendering reports
- When not provided, use hardcoded defaults (current behavior)

**Phase 2: RPC client in CLI**
- Add `reqwest` (or `ureq` for sync) as dependency to `cli`
- Add `stellar-xdr` for XDR encoding/decoding
- Implement `fetch_limits(rpc_url) -> SorobanLimits`:
  - Construct 6 `LedgerKey::ConfigSetting` XDR keys
  - Call `getLedgerEntries` with batched keys
  - Decode `ConfigSettingEntry` responses
  - Map to `SorobanLimits` struct
  - On error, log warning and fall back to hardcoded defaults

**Phase 3: CLI integration**
- If `--rpc-url` is set and fetch succeeds, use fetched limits
- If `--rpc-url` is not set or fetch fails, use hardcoded defaults
- Print source of limits in report header (fetched vs hardcoded)

### Dependencies to Add

```toml
# cli/Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }  # sync HTTP
stellar-xdr = { version = "21", features = ["std"] }  # XDR types
base64 = "0.22"  # XDR encoding
```

### Code Structure

```
cli/src/
├── main.rs              # Add --rpc-url flag
├── report.rs            # Pass SorobanLimits from main
├── html_report.rs       # (no changes)
├── limits.rs            # Add fetch_limits(), keep hardcoded defaults
└── rpc.rs               # NEW: RPC client, XDR decoding
```

### Key Implementation Details

```rust
// cli/src/rpc.rs (sketch)
use stellar_xdr::{LedgerKey, ConfigSettingId, ConfigSettingEntry, ReadXdr};

pub async fn fetch_network_limits(rpc_url: &str) -> anyhow::Result<SorobanLimits> {
    let keys = [
        ConfigSettingId::ContractComputeV0,
        ConfigSettingId::ContractLedgerCostV0,
        ConfigSettingId::ContractEventsV0,
        ConfigSettingId::ContractBandwidthV0,
    ];

    let xdr_keys: Vec<String> = keys.iter()
        .map(|id| {
            let key = LedgerKey::ConfigSetting(LedgerKeyConfigSetting {
                config_setting_id: *id,
            });
            let bytes = key.to_xdr(Limits::none())?;
            Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
        })
        .collect::<anyhow::Result<_>>()?;

    let response = reqwest::blocking::Client::new()
        .post(rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getLedgerEntries",
            "params": { "keys": xdr_keys }
        }))
        .send()?
        .json::<RpcResponse>()?;

    // Decode each entry's XDR and map to SorobanLimits
    // ... (decode ConfigSettingEntry variants)
}
```

---

## Verification Plan

1. **Unit test** — Mock RPC response with known XDR bytes, verify decoded `SorobanLimits`
2. **Integration test** — Call testnet RPC (https://soroban-testnet.stellar.org), verify limits are reasonable
3. **Manual test** — Run `soroban-cost-cli report --rpc-url https://soroban-testnet.stellar.org --manifest-path ...` and verify output shows "fetched from network"
4. **Fallback test** — Run with invalid RPC URL, verify hardcoded defaults are used with warning

---

## Protocol Version Tracking

To detect staleness of hardcoded defaults, we should record:
- The protocol version the hardcoded values were last verified against
- The ledger sequence when network limits were fetched (from `latestLedger` in RPC response)

This could be added to `SorobanLimits`:

```rust
pub struct SorobanLimits {
    // ... existing fields ...
    pub source: LimitSource,  // Hardcoded(protocol_version) | Fetched { ledger_seq, timestamp }
}

pub enum LimitSource {
    Hardcoded(u32),  // protocol version
    Fetched {
        ledger_seq: u64,
        fetched_at: std::time::Instant,
    },
}
```

---

## Open Questions

1. **Should CLI be sync or async?** — Currently sync. Adding `--rpc-url` means we need an HTTP call. Options:
   - Use `ureq` (sync HTTP) — simplest, no async runtime needed
   - Use `reqwest` with `blocking` feature — same as above
   - Add `tokio` runtime — more complex but future-proof
   
   **Recommendation:** Use `ureq` or `reqwest::blocking` to keep CLI synchronous.

2. **Should harness also fetch limits?** — The harness runs inside `cargo test` (Wasm-like env). RPC calls aren't possible there. Limits fetching should be CLI-only.

3. **Caching strategy?** — Config settings change rarely (protocol upgrades). Could cache to a file with TTL. **Recommendation:** Skip caching for v0.1; add later if needed.

4. **Which RPC endpoint to use by default?** — Could auto-detect from `SorobanRpc` in `network_passphrase`, but that adds complexity. **Recommendation:** Require explicit `--rpc-url` for v0.1.

---

## References

- [Soroban RPC `getLedgerEntries` docs](https://developers.stellar.org/docs/soroban/rpc-api#fetches-the-current- Ledger-entries-given-their-keys)
- [Soroban Cost Estimator crate](https://crates.io/crates/soroban-cost-estimator) — existing implementation of this pattern
- [Stellar XDR types](https://docs.rs/stellar-xdr/) — `ConfigSettingEntry`, `LedgerKey`, etc.
- [Soroban Network Config](https://developers.stellar.org/docs/smart-contracts/limitations#network-configurations)
