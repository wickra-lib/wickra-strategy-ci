# Benchmarks

Placeholder — real numbers are filled in from the `strategy-ci-bench` criterion
suite (P-SCI-5.6) once the core is in place.

The benchmarks will cover:

- **Single test** — parse a `StrategyTest`, run one backtest through the engine,
  and evaluate its assertions and properties.
- **Suite** — run a directory of tests, comparing the parallel (rayon) path
  against the sequential path.
- **Fuzz** — the per-run overhead of the seeded data perturbation generator.

Numbers are measured on the CI reference runner and reported as median ± MAD.
