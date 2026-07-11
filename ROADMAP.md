# Roadmap

Wickra Strategy-CI is pre-1.0. The direction below is indicative, not a
commitment; issues and discussions shape priorities.

## Now (0.1.x)

- The core test runner: golden pinning with tolerances, invariant properties,
  and seeded fuzz perturbations.
- Ten-language bindings over the C ABI hub, with a cross-language golden that is
  byte-identical everywhere.
- The reference CLI (`run` / `bless`) and the composite GitHub Action.

## Next

- More built-in properties (drawdown bounds, exposure limits, trade-count
  windows, monotonic equity checks).
- More fuzz perturbations (gaps, spikes, reordering, resampling) with documented
  determinism guarantees.
- JUnit-XML result output so CI systems render per-test status natively.
- Optional verifiable report hashes behind the `proof` feature.

## Later

- A crates.io / PyPI / npm / NuGet / Maven / Go / R release once the API is
  stable (release is USER-GO gated).
