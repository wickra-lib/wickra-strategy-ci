<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-strategy-ci)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codeql.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/release.svg)](https://github.com/wickra-lib/wickra-strategy-ci/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/crates.svg)](https://crates.io/crates/wickra-strategy-ci-cli)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/pypi.svg)](https://pypi.org/project/wickra-strategy-ci/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/npm.svg)](https://www.npmjs.com/package/wickra-strategy-ci)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/nuget.svg)](https://www.nuget.org/packages/Wickra.StrategyCi)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-strategy-ci)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-strategy-ci-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-strategy-ci)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/provenance.svg)](https://github.com/wickra-lib/wickra-strategy-ci/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/docs.svg)](https://wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/verified.svg)](golden/)

---

# Wickra Strategy-CI

**Jest for trading strategies.** Golden-pin your strategy's backtest report,
catch regressions in CI, and property-test it against fuzzed market data — in ten
languages, plus a reusable composite GitHub Action.

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib).** Strategy-CI
> is the test harness for the deterministic
> [wickra-backtest](https://github.com/wickra-lib/wickra-backtest) engine: it runs
> a strategy through the engine, pins the resulting `BacktestReport`, and fails
> the build when the numbers drift.

```bash
# Run a directory of strategy tests against a directory of OHLCV data.
# Exits non-zero the moment a report drifts, so CI fails on it.
wickra-strategy-ci run tests/ --data data/

# Re-pin the goldens after a change you meant to make.
wickra-strategy-ci bless tests/ --data data/
```

## Why

A backtest is only trustworthy if it is reproducible. Wickra's engine is
deterministic, so a strategy's report is a stable artifact you can pin — like a
snapshot test. Strategy-CI turns that into a workflow:

- **Golden tests** — pin a strategy's `BacktestReport` and fail when it changes
  beyond a tolerance you set — absolute or relative, per field, with exact
  equality as the default when you set none.
- **Property tests** — assert invariants that must hold for *any* run: no field
  is NaN or infinite, drawdown stays inside a bound, the trade count clears a
  floor, Sharpe or PnL clears a threshold, a named field stays in range.
- **Fuzz tests** — perturb the input data with a seeded PRNG and re-run, catching
  strategies that only work on one specific history.

## A test is a file, not a function

There is no test API to learn. A test is JSON: which strategy, over which
dataset, pinned to which report, under which tolerances.

```jsonc
{
  "id": "crossover",
  "dataset_ref": "sym-04",            // resolves to <data>/sym-04.csv
  "strategy": { /* opaque StrategySpec, forwarded to wickra-backtest */ },
  "expected": { /* the pinned BacktestReport — written by `bless` */ },
  "tolerances": {
    "*": { "kind": "rel", "value": 0.0001 },        // default for every field
    "metrics.sharpe": { "kind": "abs", "value": 0.01 }
  },
  "property_checks": [
    { "kind": "no_nan" },
    { "kind": "max_drawdown_le", "value": 1.0 }
  ],
  "fuzz": { "seed": 42, "runs": 8,
            "perturbation": { "kind": "jitter", "amount": 0.001 } }
}
```

Write the strategy, run `bless` once to pin the report, and commit the file. From
then on `run` fails the build whenever the numbers move further than you allowed.
Working examples live in [`golden/tests/`](golden/tests/).

```bash
wickra-strategy-ci list  tests/                 # the test ids found under a path
wickra-strategy-ci run   tests/ --data data/    # --format json for machine output
wickra-strategy-ci bless tests/ --data data/    # re-pin after an intended change
```

## As a GitHub Action

Run your strategy tests on every push — a failing test fails the workflow:

```yaml
- uses: wickra-lib/wickra-strategy-ci@v1
  with:
    tests: tests/
    data: data/
```

See [docs/GITHUB_ACTION.md](docs/GITHUB_ACTION.md) for the full inputs/outputs.

## Use in any language

The core is exposed as a JSON-over-C-ABI data API in ten languages: Rust, Python,
Node.js and WASM natively, plus C, C++, C#, Go, Java and R over the C ABI hub. A
`Session` handle plus `command(json) -> json` and `version` is the whole surface;
the same test JSON produces a byte-identical result in every binding.

```bash
cargo add wickra-strategy-ci           # Rust
pip install wickra-strategy-ci         # Python
npm install wickra-strategy-ci         # Node.js
dotnet add package Wickra.StrategyCi   # C#
go get github.com/wickra-lib/wickra-strategy-ci/bindings/go   # Go
```

Java ships to Maven Central (`org.wickra:wickra-strategy-ci`), R to r-universe
(`wickrastrategyci`), and the C ABI ships as a per-platform library with a
vendored header. See each binding's `README.md` under [`bindings/`](bindings/).

## How it works

A `StrategyTest` is data, not code: a serde model carrying an opaque
`StrategySpec` sub-JSON. Strategy-CI forwards that spec verbatim to
`wickra-backtest::run`, takes the returned `BacktestReport`, and asserts it
against the test's expectations and properties. Because the engine is
deterministic and every binding forwards the core's response string unchanged,
results are reproducible byte-for-byte across languages and between the parallel
(rayon) and sequential (WASM) execution paths.

The diff works on **numeric leaves**. Both reports are flattened to a sorted map
of numbers — `metrics.sharpe`, `equity[3].equity` — rounded to eight decimals and
compared field by field, reporting mismatches, fields that vanished and fields
that appeared. Strings, booleans and nulls are not pinned, so a report field that
is text is outside what a golden can catch.

## Benchmarks

A suite is cheap enough to gate every pull request. Median wall-clock for
`run_suite`, parallel path, from the `strategy-ci-bench` criterion suite:

| Dataset | Tests | Suite | Per test |
|---------|-------|-------|----------|
| small (200 bars)  | 100  | 13.4 ms | ~134 µs |
| small (200 bars)  | 1000 | 143 ms  | ~143 µs |
| large (2000 bars) | 100  | 156 ms  | ~1.56 ms |
| large (2000 bars) | 1000 | 1.24 s  | ~1.24 ms |

Per-test cost is dominated by the engine walking the price history — roughly
linear in bar count, near-flat in test count once the rayon pool is saturated.
The golden diff and property checks are `O(fields)` on top. A `fuzz` axis
multiplies a test's cost by its `runs`. Full method and caveats in
[BENCHMARKS.md](BENCHMARKS.md); reproduce with `cargo bench -p strategy-ci-bench`.

## Requirements

| To use | You need |
|--------|----------|
| The CLI or the GitHub Action | Nothing — the action installs a prebuilt binary, or builds from git as a fallback. |
| Rust | 1.86 or newer (workspace MSRV). |
| Python | 3.9 or newer. |
| Node.js | 22 or newer. |
| Go | 1.23 or newer. |
| Java | 22 or newer. |
| C / C++ / C# / R | The C ABI library plus its vendored header; see each binding's `README.md`. |

Building from source additionally needs a Rust toolchain; the polyglot bindings
need their own toolchain (`maturin`, `napi`, `wasm-pack`, `dotnet`, `go`, Maven,
`R CMD`) only for the binding you are building.

## Project layout

```
crates/strategy-ci-core     the runner: model, tolerances, properties, fuzz, session
crates/strategy-ci-cli      the reference CLI (run / bless / list / version)
crates/strategy-ci-bench    criterion benchmarks
bindings/c                  the C ABI hub — every non-native binding goes through it
bindings/python             PyO3 native binding
bindings/node               napi-rs native binding
bindings/wasm               wasm-bindgen binding (sequential path)
bindings/{csharp,go,java,r} thin clients over the C ABI
golden/                     cross-language fixtures: tests, data, expected reports
examples/                   one runnable example per binding
fuzz/                       cargo-fuzz targets (its own detached workspace)
action.yml                  the composite GitHub Action
```

## Building everything from source

```bash
cargo build --workspace --all-features        # core, CLI, bench, native bindings
cargo build -p wickra-strategy-ci-c --release # the C ABI library + header

( cd bindings/python && maturin develop --release )
( cd bindings/node   && npm ci && npm run build )
( cd bindings/wasm   && wasm-pack build --target web )
( cd bindings/java   && mvn -q package )
( cd bindings/csharp && dotnet build )
```

The C ABI library is the prerequisite for the C, C++, C#, Go, Java and R
bindings: build it first, then point `WKSTRATEGYCI_LIB` and `WKSTRATEGYCI_INC` at
the resulting library and `bindings/c/include`.

## Testing

```bash
cargo test --workspace --all-features         # core, integration, C ABI
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check

( cd bindings/python && pytest tests -q )
( cd bindings/node   && npm test )
( cd bindings/go     && go test ./... )
```

Every binding runs the **same** `golden/` fixtures through its own
`command(json) -> json` surface, so a passing suite is evidence that the
languages agree byte-for-byte — not just that each one runs. The CLI is covered
end-to-end against `golden/tests` in CI.

## Ecosystem

Strategy-CI is one repo in the [Wickra](https://github.com/wickra-lib) family:

| Repo | What it does |
|------|--------------|
| [wickra](https://github.com/wickra-lib/wickra) | The indicator core — 514 streaming indicators, O(1) per tick, in ten languages. |
| [wickra-backtest](https://github.com/wickra-lib/wickra-backtest) | The deterministic engine whose `BacktestReport` this repo pins. |
| [wickra-data](https://github.com/wickra-lib/wickra) | Candle types and CSV/exchange loading. |
| [wickra-proof](https://github.com/wickra-lib/wickra-proof) | Verifiable report hashes — the optional `proof` feature here. |
| [wickra-synth](https://github.com/wickra-lib/wickra-synth) | Deterministic synthetic market data, useful as fuzz input. |
| [wickra-exchange](https://github.com/wickra-lib/wickra-exchange) | Live and historical exchange connectivity. |

## Documentation

See [docs/](docs/) — [`TESTS.md`](docs/TESTS.md) for the test model,
[`TOLERANCES.md`](docs/TOLERANCES.md) for the golden diff,
[`PROPERTIES.md`](docs/PROPERTIES.md) for the invariants,
[`FUZZING.md`](docs/FUZZING.md) for the perturbations,
[`GITHUB_ACTION.md`](docs/GITHUB_ACTION.md) for the action, and
[`Cookbook.md`](docs/Cookbook.md) for task-shaped recipes.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Code of Conduct](CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](SECURITY.md).

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.

## Disclaimer

`wickra-strategy-ci` is research and engineering tooling, not financial advice. A
passing test attests only that a strategy's backtest report matches its pinned
expectation under the given data — it makes no claim about the quality,
profitability or future performance of any strategy. Trading carries risk; you
are responsible for your own decisions.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-strategy-ci">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-strategy-ci/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-strategy-ci/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-strategy-ci star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/star-history.svg">
</p>
