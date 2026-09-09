# TODO — Post v1.0.0

## Completed in v1.0.0

- [x] 5-dimension resource profiling (CPU, memory, read bytes, write bytes, events size)
- [x] CLI with `--check` flag for CI pass/fail
- [x] GitHub Action for PR cost reporting
- [x] Malformed JSON resilience with warnings
- [x] Clear error messages for missing manifests
- [x] HTML report generation
- [x] crates.io publication (soroban-cost-cli + soroban-cost-harness)
- [x] Validated against stellar/soroban-examples liquidity pool

## Out of Scope (v2.0+)

- [ ] Transaction size measurement (requires XDR serialization, not exposed by InvocationResources)
- [ ] Live network config fetching (soroban-cost-estimator covers this)
- [ ] simulateTransaction mode (soroban-cost-estimator covers this)
- [ ] Network config drift tracking (soroban-cost-estimator covers this)
