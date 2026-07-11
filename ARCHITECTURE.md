# Architecture

Wickra Strategy-CI is a test runner for trading strategies, built as one
data-driven core with thin language bindings, a reference CLI, and a composite
GitHub Action.

## Workspace layout

```
crates/
  strategy-ci-core/    the library: model, tolerances, properties, fuzz, runner, session
  strategy-ci-cli/     the reference `wickra-strategy-ci` binary
  strategy-ci-bench/   criterion benchmarks
bindings/
  c/                   the C ABI hub (cdylib + staticlib) → C/C++/C#/Go/Java/R
  python/              PyO3 native module
  node/                napi native module
  wasm/                wasm-bindgen module (sequential; no rayon)
golden/                pinned tests + data + expected results (cross-language golden)
examples/              per-language usage
action.yml             the composite GitHub Action wrapping the CLI
fuzz/                  cargo-fuzz targets (detached workspace)
```

## The core

`strategy-ci-core` is the whole engine. Its axes:

- **Model** (`model.rs`) — `StrategyTest` (the unit of testing), `TestResult`,
  and the assertions over a `BacktestReport`. The `strategy` field is an **opaque
  `StrategySpec` sub-JSON**: it is never parsed here, only forwarded to
  `wickra-backtest`.
- **Tolerances** (`tolerance.rs`) — how a report field is compared to its pinned
  value: exact, absolute, relative, or ULP.
- **Properties** (`property.rs`) — invariants that must hold for any run,
  independent of the golden value.
- **Fuzz** (`fuzz.rs`) — a seeded, deterministic data perturbation generator
  (`rand_pcg`, never `thread_rng`) that re-runs a test over many synthetic
  histories.
- **Runner** (`runner.rs`) — executes a test: forward the spec + data to
  `wickra-backtest::run`, collect the `BacktestReport`, evaluate assertions,
  properties and fuzz runs into a `TestResult`.
- **Session** (`session.rs`) — the `command(json) -> json` boundary every binding
  drives. One serializer, forwarded verbatim, so every language and both the
  parallel (rayon) and sequential (WASM) paths produce byte-identical output.

## The system under test

The `wickra-backtest` engine is a git dependency. Strategy-CI **calls** its
`run(&StrategySpec, &[Candle]) -> BacktestReport` and pins the result; it does not
reimplement any backtest logic. The `StrategySpec` schema and the determinism
guarantee live in `wickra-backtest`.

## Determinism

Determinism is the moat: a test's result is byte-identical across all ten
languages and between the rayon and WASM execution paths. That is enforced by
using `BTreeMap` on every output path, sorting result vectors by a stable key,
rounding report fields deterministically before comparison, and seeding every
PRNG explicitly. See `docs/TESTS.md` and `docs/TOLERANCES.md`.
