# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **JSON float parsing was not bit-exact, in a project whose claim is
  byte-identical results.** `serde_json`'s default float parser is fast but can
  land one ULP from the value the text names, so `parse(serialize(x)) != x` for
  some inputs. The `test_parse` fuzz target found it on its first run after the
  new workflows landed: `1888888888888288888888855` parsed to
  `1.8888888888882886e24`, serialized, and came back as `1.888888888888289e24`.
  A golden is text that is re-parsed on every run, so an inexact parser
  undermines both reproducibility between runs and identity across the ten
  bindings. The workspace and the fuzz crate now enable `serde_json`'s
  `float_roundtrip` feature, with a unit test pinning the property, since nothing
  else in the build would notice the feature being dropped.
- **`js-yaml` 4.3.0 carried a high-severity advisory** (GHSA-5p4m-2wfm-xmqj,
  CVSS 7.5), reachable as a transitive development dependency of
  `@napi-rs/cli`. Found by the osv-scanner job the moment it was wired up;
  `bindings/node/package-lock.json` now resolves 4.3.2.
- **The shipped R test reached outside the package.** `tests/run_tests.R` walked
  up to ten directories looking for `golden/`. `R CMD check` runs the shipped
  tests inside an unpacked tarball, where no repository lies above the package,
  so the cross-language comparison either skipped silently on every real check
  run or bound to an unrelated directory that happened to carry that name. It is
  now split: `run_tests.R` is self-contained, and the golden comparison lives in
  `tests/golden_cross_language.R`, which is `.Rbuildignore`d and run from CI
  against the checkout.
- **The npm platform packages would have published without their licence
  texts.** All seven manifests name `MIT OR Apache-2.0`, but an SPDX expression
  is a reference to two documents, not the documents. The `files` field now lists
  them, and a release step stages the copies and proves with `npm pack
  --dry-run` that they are in each tarball before anything is published.
- **The main npm package shipped the platform stubs inside itself.** Its `files`
  listed `npm` and `*.node`; the native binaries reach a consumer through the
  per-platform packages in `optionalDependencies`, so the tarball is now the JS
  loader, the types and the licences.
- **The Linux npm stubs did not declare `libc`.** Without it npm installs the
  gnu package on a musl system, where the binary cannot load. Both gnu stubs now
  declare `glibc`.
- `bindings/r/DESCRIPTION` declared no lower bound on R.

- **`wickra-data` was declared and used by nothing.** It sat in
  `[workspace.dependencies]` pinned at `0.9` while the published crate is at
  `1.0.4`, so it was simultaneously dead weight and stale. No crate imported it:
  the CLI reads its own CSV and `Candle` is re-exported from the engine. Adding
  it back would mean two crates defining the same row, so it is removed rather
  than bumped.
- Neither published crate declared `[package.metadata.docs.rs] all-features`, so
  docs.rs would have rendered the default feature set only, leaving the optional
  `proof` API and the rayon-backed `parallel` path invisible to a reader.
- Neither published crate carried the licence texts it names. `cargo` decides
  what to package from git, so an untracked copy makes `cargo publish` refuse the
  tree and a gitignored one is dropped from the tarball; the copies are committed
  beside each manifest.

### Added

- **`bindings/r/configure` and `configure.win`.** The R binding had neither, so
  it could only ever build against a C ABI already present in the environment —
  which is what CI provides and what a user installing from r-universe does not
  have. Both now download the `wickra-strategy-ci-c-<triple>.tar.gz` asset
  matching `DESCRIPTION: Version`, stage it into `src/`, and bake an rpath
  ($ORIGIN / @loader_path) so the bundled library is found after install.
  `src/Makevars.in` and `src/install.libs.R` come with them; `src/Makevars` is
  now generated rather than committed.
- **`bindings/c/include/wickra_strategy_ci.hpp`** — the C++ hull. The C ABI hands
  out a handle that must be freed exactly once and a two-call buffer protocol for
  `command`, and every C++ caller re-implemented both by hand. `Session` is a
  move-only RAII owner and `command` returns a `std::string`. `examples/c/run.cpp`
  now uses it, which is what keeps it compiling.
- `bindings/csharp/README.md` — the binding-level landing page. The one beside
  the `.csproj` is packaged into the `.nupkg` and is a different document.
- `examples/wasm/run.mjs` — WASM was the only binding without a runnable example.



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
