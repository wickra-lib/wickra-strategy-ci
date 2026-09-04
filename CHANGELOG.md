# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **Governance and community docs described a different product.** `SECURITY.md`,
  `SUPPORT.md`, `GOVERNANCE.md`, `CONTRIBUTING.md`, the bug-report issue template
  and the pull-request template referred to a `ScanSpec`, a sample "universe" and
  an optional `live` feature — none of which exist in this repository. They now
  describe the `StrategyTest` model, recorded datasets, and the actual `parallel`
  and `proof` features.
- **`deny.toml` carried a dead licence exception.** The `CDLA-Permissive-2.0`
  allowance for `webpki-roots` was justified by a TLS stack that this crate never
  pulls in; `webpki-roots` is not in `Cargo.lock`. The exception list is now
  empty.
- `README.md` and `ARCHITECTURE.md` advertised four tolerance kinds ("exact,
  absolute, relative, or ULP"). The runner implements `abs` and `rel`, falling
  back to exact equality when a field names no tolerance; there is no ULP mode.
- `README.md` listed invariant properties that do not exist (`equity never
  negative`); it now names the seven that do.
- `CHANGELOG.md` credited the composite action with a `@v1` moving major tag; no
  tag has been published yet.
- `SECURITY.md` stated that security fixes target "the most recent published
  version" as though one existed.

### Added

- `LICENSES/MIT.txt` and `LICENSES/Apache-2.0.txt` — the SPDX licence texts at
  their conventional paths, alongside the existing `LICENSE-MIT` /
  `LICENSE-APACHE`.
- `docs/README.md` — an index of the documentation kept beside the code, and a
  signpost to what deliberately lives elsewhere (`docs.wickra.org`, and the
  `StrategySpec` schema in `wickra-backtest`).
- `README.md` gained the sections the repository blueprint requires:
  `Benchmarks`, `Requirements`, `Project layout`, `Building everything from
  source`, `Testing` and `Ecosystem`, plus a runnable quickstart directly under
  the badges and a worked `StrategyTest` example.
- `README.md` and `ARCHITECTURE.md` now state that the golden diff compares
  numeric leaves only — strings, booleans and nulls are not pinned.


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
  a dogfooding self-test. It builds the CLI from git until the first release
  ships a binary asset.
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
