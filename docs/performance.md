# Performance Notes

This covers the performance of the profiler tool itself — not to be confused with the resource-usage reports it *produces* for your contract.

## Where time goes when you run `soroban-cost-cli report`

1. **`cargo test` compilation** — dominates wall-clock time for most projects. This is inherent to running the target's real test suite and isn't something the CLI can meaningfully speed up beyond what `cargo`'s own incremental compilation already does.
2. **Test execution** — proportional to however many `record()`-wrapped calls and other tests the target project has.
3. **Output parsing** — negligible; scanning stdout lines for a fixed prefix and deserializing small JSON objects is not a bottleneck even for hundreds of records.
4. **Report rendering** — negligible for both the terminal table and the HTML report.

In practice, if `soroban-cost-cli report` feels slow, it's almost always #1 or #2, not the profiler's own overhead.

## Measurement accuracy

`record()` resets the budget to unlimited before each wrapped call specifically so that:

- Setup code in earlier parts of the same test doesn't pollute the measurement.
- Multiple `record()` calls in the same test function each report *only* the cost of their own closure, not a running total.

If you see costs that seem too high, check whether the closure passed to `record()` includes setup/teardown that should be outside it.

## Known limitations (v0.1)

- Only CPU instructions and memory bytes are measured — see `docs/roadmap.md` for the v0.2 plan to add ledger I/O, tx size, and events.
- The CLI re-runs the entire `cargo test` invocation on every `report` call; there's no caching of previous runs. For large test suites this means iterating on cost optimization means re-paying full compile+test time each pass. Contributions to make this cheaper (e.g. `--filter` to run only cost-relevant tests) are welcome — see open issues.

## Contributing perf improvements

If you're profiling the profiler itself (meta, but valid), please include before/after timings on `examples/token-example` in your PR description, and ideally on a larger real-world contract if you have one available.
