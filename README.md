<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Strategy-CI — golden-pin your strategy's backtest report, catch regressions in CI, and property-test against fuzzed data, in ten languages plus a reusable GitHub Action" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-strategy-ci)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/ci.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codeql.svg)](https://github.com/wickra-lib/wickra-strategy-ci/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-strategy-ci)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-strategy-ci)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/best-practices.svg)](https://www.bestpractices.dev/)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/provenance.svg)](https://github.com/wickra-lib/wickra-strategy-ci/attestations)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-strategy-ci/docs.svg)](https://wickra.org)

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

## Why

A backtest is only trustworthy if it is reproducible. Wickra's engine is
deterministic, so a strategy's report is a stable artifact you can pin — like a
snapshot test. Strategy-CI turns that into a workflow:

- **Golden tests** — pin a strategy's `BacktestReport` and fail when it changes
  beyond a tolerance you set (exact, absolute, relative, or ULP).
- **Property tests** — assert invariants that must hold for *any* run (equity
  never negative, trade count within bounds, Sharpe finite, …).
- **Fuzz tests** — perturb the input data with a seeded PRNG and re-run, catching
  strategies that only work on one specific history.

## Quickstart

```bash
# Run a directory of strategy tests against a dataset directory.
wickra-strategy-ci run tests/ --data data/

# Re-bless the golden reports after an intentional change.
wickra-strategy-ci bless tests/ --data data/
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

## Documentation

See [docs/](docs/): `TESTS.md`, `PROPERTIES.md`, `TOLERANCES.md`, `FUZZING.md`,
and `GITHUB_ACTION.md`.

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
