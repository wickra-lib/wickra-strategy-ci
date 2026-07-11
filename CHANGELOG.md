# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`strategy-ci-core`** — the data-driven test runner: a serde `StrategyTest`
  model carrying an opaque `wickra-backtest` `StrategySpec`, golden-diff with
  per-field `abs`/`rel` tolerances, invariant property checks (`no_nan`,
  `monotone_equity`, `max_drawdown_le`, `min_trades_ge`, `sharpe_ge`, `pnl_ge`,
  `field_in_range`) with `metrics.<name>` path resolution, and a seeded
  `rand_pcg` fuzz axis (`jitter`/`dropout`/`gap_shock`). Optional `proof` feature
  surfaces a BLAKE3 report hash.
- **`wickra-strategy-ci` CLI** — `run` (non-zero exit on any failure — the CI
  gate), `bless`, `list`, `version`, text and JSON output.
- **Ten-language bindings** over one `Session`/`command_json` boundary: Rust,
  Python (PyO3), Node.js (napi-rs) and WASM (wasm-bindgen) natively, plus C, C++,
  C#, Go, Java and R over a C ABI hub — byte-identical results across all.
- **Composite GitHub Action** (`action.yml`) that installs the released CLI and
  runs a strategy-test directory, failing the workflow on any failing test, with
  a `@v1` moving major tag and a dogfooding self-test.
- **Golden corpus** (`golden/`) run by every binding for cross-language
  byte-identity, four integration test suites, four cargo-fuzz targets, and a
  criterion benchmark.
- Full CI across all ten languages, CodeQL, OpenSSF Scorecard, zizmor, a nightly
  benchmark, a weekly link check, metadata-drift audit, and a tag-gated release
  pipeline to crates.io, PyPI, npm, NuGet, Maven Central, r-universe and Go.
- Project scaffolding: the workspace manifest, dual `MIT OR Apache-2.0` license,
  supply-chain and link config (`deny.toml`, `lychee.toml`, `osv-scanner.toml`,
  `repo-metadata.toml`), and the governance and community docs.

[Unreleased]: https://github.com/wickra-lib/wickra-strategy-ci/commits/main
