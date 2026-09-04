# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The composite action was advertised at a tag that will never exist.** README,
  `docs/GITHUB_ACTION.md`, `docs/Cookbook.md` and the generated release notes all
  showed `uses: wickra-lib/wickra-strategy-ci@v1`. The `major-tag` job derives
  the moving tag from the release, so `v0.1.0` yields **`v0`** — a workflow
  pinning `@v1` would fail to resolve. All four now pin the exact release, and
  say why a floating `@v0` is the wrong default while a `0.x` minor is allowed to
  break compatibility.
- Every job now carries a `timeout-minutes` backstop; 22 of them had none and
  inherited GitHub's six-hour default, so a wedged job held a runner for a
  working day.
- `ci.yml` built pull requests against any target branch; it now filters on
  `main`.
- The two `action-selftest` checkouts ran without `persist-credentials: false`,
  leaving the job token in the runner's git config for the rest of the job. The
  one remaining checkout that keeps credentials is `major-tag`, which pushes a
  tag and needs them.
- **Eight shell defects actionlint found on its first run**, in code that had
  been shipping. Three publish steps used `A && B || C`, which reads as
  if-then-else and is not: the fallback also runs when the publish succeeds and
  the `echo` after it fails, so a successful publish could have been reported as
  "already published" — or worse, a real failure swallowed. All three are now
  spelled as `if`/`elif`/`else`. Two `local x=$(...)` assignments masked the
  command's exit status behind `local`'s own (SC2155), and the release-asset
  count parsed `ls` output (SC2012).
- Pin comments on `Swatinem/rust-cache` and `lycheeverse/lychee-action` read
  `# v2`, too coarse for Dependabot to act on. Both now name the patch version,
  and rust-cache moves to the same v2.9.2 the rest of the organisation pins.


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
- The feature-request issue template asked for "a new comparator, a cross-section
  or breadth metric" and described "the screen you can't express today" — text
  carried over from `wickra-screener`. It now asks about tolerances, properties
  and perturbations.

### Added

- **`osv-scanner` now runs in CI.** `osv-scanner.toml` existed as a VEX record —
  advisories assessed as not affecting this project, reasoning beside each — and
  no workflow ever consulted it. `cargo-deny` covers the Rust graph only; this
  covers the other six ecosystems (npm, PyPI, Maven, NuGet, Go modules, R), which
  had no vulnerability scanning at all.
- `actionlint.yml` — workflow correctness, the half zizmor does not cover:
  unknown contexts, invalid `needs`, unused matrix keys, and through its bundled
  shellcheck, the shell inside every `run:` block. The release pipeline alone
  carries several hundred lines of it, unchecked until now.
- `codspeed.yml` — instruction counts on every pull request. `bench.yml` runs on
  a schedule and prints numbers a person has to read, so a regression surfaced
  only if someone looked. The bench crate now builds against
  `codspeed-criterion-compat`, which behaves as criterion does off a CodSpeed
  runner.

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
- Five issue templates the blueprint requires: `bug_report_detailed`,
  `feature_request_detailed`, `performance_regression`, `documentation` and
  `question`. The detailed bug template asks whether a defect reproduces in a
  second binding and on the sequential path, because a cross-language or
  parallel-only disagreement is the class of bug this project most needs
  reported precisely.
- `.github/PULL_REQUEST_TEMPLATE/detailed.md` — the long-form template, covering
  wire-format impact, binding parity across all ten languages, and determinism.
  GitHub offers no picker for it, so `PULL_REQUEST_TEMPLATE.md` now says how to
  reach it (`?template=detailed.md`).
- `.github/codeql/codeql-config.yml`, referenced from the CodeQL workflow, so the
  napi-derive glue in `bindings/node/src/lib.rs` stops producing an
  `access-invalid-pointer` finding per exported class while the rule stays active
  over `bindings/c/src`, where the real raw-pointer code lives.
- CodeQL now analyses **C# and Java**, the two bindings that reach the core
  across the C ABI. Both build with `build-mode: manual`; without a build no
  dependency resolves and GitHub reports the analysis as low quality.
- Dependabot now covers `fuzz/` (a detached workspace the root cargo entry cannot
  reach) and `examples/go` (its own module). Both would otherwise never see a
  dependency update.


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
