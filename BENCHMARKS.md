# Benchmarks

Measured with the `strategy-ci-bench` criterion suite:

```bash
cargo bench -p strategy-ci-bench
```

The suite benchmarks `run_suite` across the cross-product of test counts
{10, 100, 1000} and dataset sizes (`small` = 200 bars, `large` = 2000 bars), so
the numbers show how a golden-diff + property run scales with both the number of
strategy tests and the length of the price history each one backtests over. The
parallel (rayon) and sequential paths are selected by the `parallel` feature; run
with and without `--no-default-features` to compare.

## run_suite (parallel, default features)

Wall-clock for a full suite, and the derived per-test cost. Reference numbers
below were taken on the developer workstation; the authoritative figures are the
nightly `bench.yml` run on the CI reference runner. Report as median.

| Dataset | Tests | Suite | Per test |
|---------|-------|-------|----------|
| small (200 bars)  | 10   | 2.1 ms  | ~206 µs |
| small (200 bars)  | 100  | 13.4 ms | ~134 µs |
| small (200 bars)  | 1000 | 143 ms  | ~143 µs |
| large (2000 bars) | 10   | 21.2 ms | ~2.12 ms |
| large (2000 bars) | 100  | 156 ms  | ~1.56 ms |
| large (2000 bars) | 1000 | 1.24 s  | ~1.24 ms |

Per-test cost is dominated by the `wickra-backtest` engine walking the price
history — it scales roughly linearly with bar count (10× the bars ≈ 10× the
per-test time) and is near-flat in the number of tests once the rayon pool is
saturated. Strategy-CI's own golden diff, property checks and report flattening
are a small fraction on top.

## Interpreting the numbers

- A realistic CI suite of a few dozen tests over a couple of thousand bars each
  completes in well under a second — cheap enough to gate every pull request.
- The golden diff, property evaluation and tolerance flattening are `O(fields)`
  and negligible next to the backtest itself.
- The per-test `fuzz` axis multiplies a test's cost by its `runs` count, since
  each run re-backtests a perturbed dataset. Keep `runs` modest (8–16) on hot CI
  paths.

Numbers are reported as median and will drift with the engine and the runner;
treat the relative scaling, not the absolute milliseconds, as the contract.
